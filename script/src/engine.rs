use crate::rr::PyGame;
use crate::{message::ScriptMessage, Error};
use crate::{GameMethod, ScriptResult};
use common::gamedata::{GameData, Value};
use common::obj::ScriptObject;
use crossbeam_channel::{Receiver, Sender};
use rustpython_vm as vm;
use vm::function::IntoPyObject;

pub type GameMethodCaller = fn(&mut GameData, method: GameMethod) -> Value;

#[derive(Clone)]
pub struct ScriptEngine {
    ready_rx: Receiver<()>,
    start_tx: Sender<StartScript>,
    method_rx: Receiver<ScriptMessage>,
    method_result_tx: Sender<Value>,
    game_method_caller: GameMethodCaller,
}

#[derive(Clone, Debug)]
pub struct StartScript {
    id: String,
    scene: Option<String>,
}

impl ScriptEngine {
    pub fn start_init(game_method_caller: GameMethodCaller) -> Self {
        log::trace!("Start script engine initialization");
        let (ready_tx, ready_rx) = crossbeam_channel::bounded(0);
        let (start_tx, start_rx) = crossbeam_channel::bounded(0);
        let (method_tx, method_rx) = crossbeam_channel::bounded(0);
        let (method_result_tx, method_result_rx) = crossbeam_channel::bounded(0);

        std::thread::spawn(move || {
            init_script(ready_tx, start_rx, method_tx, method_result_rx);
        });

        ScriptEngine {
            ready_rx,
            start_tx,
            method_rx,
            method_result_tx,
            game_method_caller,
        }
    }

    pub fn wait_init(&self) {
        self.ready_rx
            .recv()
            .expect("Script engine initialization failed");
        log::info!("Finish script engine initialization");
    }

    pub fn start_script(&mut self, id: &str, scene: Option<String>) {
        self.start_tx
            .send(StartScript {
                id: id.into(),
                scene,
            })
            .unwrap();
    }

    pub fn ui_response(&mut self, value: Value) {
        self.method_result_tx.send(value).unwrap();
    }

    pub fn next(&mut self, gd: &mut GameData) -> ScriptResult {
        loop {
            match self.method_rx.recv().unwrap() {
                ScriptMessage::Fail => {
                    log::warn!("Script execution failed");
                    return ScriptResult::Finish;
                }
                ScriptMessage::Finish => {
                    return ScriptResult::Finish;
                }
                ScriptMessage::UiRequest(request) => {
                    return ScriptResult::UiRequest(request);
                }
                ScriptMessage::Exec(e) => {
                    let result = e(gd);
                    self.method_result_tx.send(result).unwrap();
                }
                ScriptMessage::Method(method) => {
                    let result = (self.game_method_caller)(gd, method);
                    self.method_result_tx.send(result).unwrap();
                }
            }
        }
    }
}

fn init_script(
    ready_tx: Sender<()>,
    start_rx: Receiver<StartScript>,
    method_tx: Sender<ScriptMessage>,
    method_result_tx: Receiver<Value>,
) {
    let settings = vm::PySettings {
        no_site: true,
        no_user_site: true,
        ignore_environment: true,
        isolated: true,
        ..vm::PySettings::default()
    };
    let result: Result<(), Error> = vm::Interpreter::new_with_init(settings, |vm| {
        vm.add_native_module("rr".to_owned(), Box::new(crate::rr::make_module));
        vm.add_native_module("random".to_owned(), Box::new(crate::random::make_module));
        vm::InitParameter::Internal
    })
    .enter(move |vm| {
        load_modules(vm)?;
        ready_tx.send(()).unwrap();
        script_loop(vm, start_rx, method_tx, method_result_tx)
    });

    if let Err(e) = result {
        log::error!("Script failure:\n{}", e);
        std::process::exit(1);
    }
}

fn script_loop(
    vm: &vm::VirtualMachine,
    start_rx: Receiver<StartScript>,
    method_tx: Sender<ScriptMessage>,
    method_result_rx: Receiver<Value>,
) -> Result<(), Error> {
    while let Ok(start_script) = start_rx.recv() {
        let pygame = PyGame {
            scene: start_script.scene.clone(),
            self_id: start_script.id.clone(),
            method_tx: method_tx.clone(),
            method_result_rx: method_result_rx.clone(),
        };
        if let Err(e) = call_script(vm, &start_script, pygame) {
            log::warn!(
                "Error during executing script \"{}\"\n{}",
                start_script.id,
                e
            );
            method_tx.send(ScriptMessage::Fail).unwrap();
        } else {
            method_tx.send(ScriptMessage::Finish).unwrap();
        }
    }
    Ok(())
}

fn call_script(
    vm: &vm::VirtualMachine,
    start_script: &StartScript,
    pygame: PyGame,
) -> Result<(), Error> {
    let script_obj: &ScriptObject = common::gobj::get_by_id(&start_script.id);

    let scope = vm.new_scope_with_builtins();
    scope
        .globals
        .set_item("game", pygame.into_pyobject(vm), vm)
        .map_err(|e| Error::from_py(vm, e))?;

    let script = vm.compile(
        &script_obj.script,
        vm::compile::Mode::Exec,
        start_script.id.clone(),
    )?;
    vm.run_code_obj(script, scope)
        .map_err(|e| Error::from_py(vm, e))?;

    log::trace!("Finish running script");
    Ok(())
}

fn load_modules(vm: &vm::VirtualMachine) -> Result<(), Error> {
    let code_obj = vm.compile(
        r#"import rr"#,
        vm::compile::Mode::Exec,
        "<load_modules>".to_owned(),
    )?;
    vm.run_code_obj(code_obj, vm.new_scope_with_builtins())
        .map_err(|e| Error::from_py(vm, e))?;
    Ok(())
}
