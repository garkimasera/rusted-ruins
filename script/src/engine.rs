use crate::{Error, ScriptYield};
use common::gamedata::GameData;
use common::obj::ScriptObject;
use rustpython_vm as vm;
use vm::{InitParameter, PySettings};

#[derive(Clone)]
pub struct ScriptEngine<'a> {
    vm: &'a vm::VirtualMachine,
    scope: Option<vm::scope::Scope>,
}

pub fn enter<F: FnOnce(ScriptEngine<'_>) -> R, R>(f: F) -> R {
    let settings = PySettings {
        no_site: true,
        no_user_site: true,
        ignore_environment: true,
        isolated: true,
        ..PySettings::default()
    };
    vm::Interpreter::new_with_init(settings, |vm| {
        vm.add_native_module("rr".to_owned(), Box::new(crate::rr::make_module));
        vm.add_native_module("random".to_owned(), Box::new(crate::random::make_module));
        InitParameter::Internal
    })
    .enter(|vm| {
        let script_engine = ScriptEngine { vm, scope: None };
        f(script_engine)
    })
}

impl<'a> ScriptEngine<'a> {
    pub fn start(&mut self, script_obj: &ScriptObject, name: &str) -> Result<(), Error> {
        self.start_with_input(&script_obj.script, name)
    }

    pub fn start_with_input(&mut self, input: &str, name: &str) -> Result<(), Error> {
        info!("start script {}", name);
        assert!(!self.during_exec());
        let scope = self.vm.new_scope_with_builtins();

        // Add codes to execute script
        let script_yield_defs = vm::py_compile!(file = "python/script_yield.py");
        let script_yield_defs = self.vm.new_code_object(script_yield_defs);
        self.vm
            .run_code_obj(script_yield_defs, scope.clone())
            .map_err(|e| Error::from_py(self.vm, e))?;

        let script = self
            .vm
            .compile(input, vm::compile::Mode::Exec, name.into())?;
        self.vm
            .run_code_obj(script, scope.clone())
            .map_err(|e| Error::from_py(self.vm, e))?;

        let start = self.vm.compile(
            "_rrscript_gen = rr_main()",
            vm::compile::Mode::Single,
            "<rrscript_gen>".into(),
        )?;
        self.vm
            .run_code_obj(start, scope.clone())
            .map_err(|e| Error::from_py(self.vm, e))?;

        self.scope = Some(scope);

        Ok(())
    }

    pub fn next(&mut self, gd: &mut GameData) -> Result<Option<ScriptYield>, Error> {
        let scope = self.scope.as_ref().expect("called run() before start()");
        let result = crate::gamedata::enter(gd, || crate::run::run(self.vm, scope));

        if result.is_err() {
            self.scope = None;
        }
        let result = result?;

        if result.is_none() {
            self.scope = None;
        }

        Ok(result)
    }

    pub fn during_exec(&self) -> bool {
        self.scope.is_some()
    }
}
