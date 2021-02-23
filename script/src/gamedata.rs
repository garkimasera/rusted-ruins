use common::gamedata::GameData;
use common::script::Value;
use once_cell::unsync::Lazy;
use rustpython_vm as vm;
use std::cell::RefCell;
use vm::builtins::{PyInt, PyNone, PyStrRef};
use vm::pyobject::{ItemProtocol, PyObjectRef, PyValue};
use vm::scope::Scope;
use vm::VirtualMachine;

thread_local!(
    static GAME_DATA: Lazy<RefCell<Option<GameData>>> = Lazy::new(|| RefCell::new(None));
);

pub fn enter<F, R>(gd: &mut GameData, f: F) -> R
where
    F: FnOnce() -> R,
{
    let mut result = None;
    // Temporary move GameData to static.
    take_mut::take_or_recover(gd, GameData::default, |gd| {
        GAME_DATA.with(|static_gd| {
            *static_gd.borrow_mut() = Some(gd);
        });
        result = Some(f());
        GAME_DATA.with(|static_gd| static_gd.borrow_mut().take().unwrap())
    });
    result.unwrap()
}

macro_rules! add_fn {
    ($vm:expr, $scope:expr, $f:ident) => {
        $scope.globals.set_item(
            stringify!($f),
            $vm.ctx.new_function(stringify!($f), $f),
            $vm,
        )
    };
}

pub fn add_fns(vm: &VirtualMachine, scope: &Scope) -> vm::pyobject::PyResult<()> {
    let game_data_defs = vm::py_compile!(file = "python/gamedata.py");
    let game_data_defs = vm.new_code_object(game_data_defs);
    vm.run_code_obj(game_data_defs, scope.clone())?;

    add_fn!(vm, scope, get_gvar)?;
    add_fn!(vm, scope, set_gvar_int)?;
    Ok(())
}

fn set_gvar_int(name: PyStrRef, value: i32) {
    with_gd_mut(|gd| gd.vars.set_global_var(name, Value::Int(value)));
}

fn get_gvar(name: PyStrRef, vm: &VirtualMachine) -> PyObjectRef {
    if let Some(value) = with_gd(|gd| gd.vars.global_var(name.as_ref()).cloned()) {
        match value {
            Value::Int(value) => {
                let value: PyInt = value.into();
                value.into_object(vm)
            }
            _ => todo!(),
        }
    } else {
        PyNone.into_object(vm)
    }
}

fn with_gd<F, R>(f: F) -> R
where
    F: FnOnce(&GameData) -> R,
{
    GAME_DATA.with(|static_gd| f(static_gd.borrow().as_ref().expect("GAME_DATA is not set")))
}

fn with_gd_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut GameData) -> R,
{
    GAME_DATA.with(|static_gd| {
        f(static_gd
            .borrow_mut()
            .as_mut()
            .expect("GAME_DATA is not set"))
    })
}
