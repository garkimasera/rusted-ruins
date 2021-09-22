//! Management codes for GameData

use common::gamedata::{GameData, Value};
use once_cell::sync::Lazy;
use once_cell::unsync::Lazy as UnsyncLazy;
use rustpython_vm as vm;
use std::cell::RefCell;
use std::convert::TryInto;
use std::sync::RwLock;
use vm::builtins::PyInt;
use vm::VirtualMachine;
use vm::{IntoPyObject, PyObjectRef, PyResult};

thread_local!(
    static GAME_DATA: UnsyncLazy<RefCell<Option<GameData>>> =
        UnsyncLazy::new(|| RefCell::new(None));
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

pub fn with_gd<F, R>(f: F) -> R
where
    F: FnOnce(&GameData) -> R,
{
    GAME_DATA.with(|static_gd| f(static_gd.borrow().as_ref().expect("GAME_DATA is not set")))
}

pub fn with_gd_mut<F, R>(f: F) -> R
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

pub fn value_to_py(vm: &VirtualMachine, value: Value) -> PyObjectRef {
    match value {
        Value::Bool(value) => value.into_pyobject(vm),
        Value::Int(value) => value.into_pyobject(vm),
    }
}

pub fn py_to_value(vm: &VirtualMachine, pyvalue: PyObjectRef) -> PyResult<Value> {
    let value = if let Some(i) = pyvalue.payload::<PyInt>() {
        let i: i64 = i.as_bigint().try_into().unwrap();
        Value::Int(i)
    } else {
        return Err(vm.new_type_error(format!("invalid type value \"{}\" for set_gvar", pyvalue)));
    };
    Ok(value)
}

pub(crate) static GAME_METHODS: Lazy<RwLock<Option<GameMethods>>> = Lazy::new(|| RwLock::new(None));

pub fn set_game_methods(game_methods: GameMethods) {
    *GAME_METHODS.write().unwrap() = Some(game_methods);
}

/// Game methods usable from scripts.
pub struct GameMethods {
    pub has_empty_for_party: fn(&GameData) -> bool,
    pub has_item: fn(&GameData, &str) -> Option<u32>,
    pub gen_dungeons: fn(&mut GameData),
    pub gen_party_chara: fn(&mut GameData, &str, u32) -> bool,
    pub receive_quest_rewards: fn(&mut GameData) -> bool,
    pub receive_item: fn(&mut GameData, &str, u32),
    pub receive_money: fn(&mut GameData, u32),
    pub remove_item: fn(&mut GameData, &str, u32) -> Result<(), ()>,
    pub resurrect_party_members: fn(&mut GameData),
}

macro_rules! call_game_method {
    ($name:ident) => {
        ($crate::gamedata::GAME_METHODS
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .$name)
    };
}
