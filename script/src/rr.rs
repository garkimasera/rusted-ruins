//! Python module for game script

use rustpython_vm::pymodule;

pub(crate) use _rr::make_module;

#[pymodule(name = "rr")]
mod _rr {
    use crate::gamedata::{py_to_value, value_to_py, with_gd, with_gd_mut};
    use rustpython_vm as vm;
    use std::convert::TryInto;
    use vm::builtins::{PyNone, PyStrRef};
    use vm::{IntoPyObject, PyObjectRef, PyResult, PyValue, VirtualMachine};

    #[pyfunction]
    fn response(vm: &VirtualMachine) -> Option<PyObjectRef> {
        with_gd(|gd| gd.script_exec.response.clone()).map(|value| value_to_py(vm, value))
    }

    #[pyfunction]
    fn self_id() -> String {
        with_gd(|gd| gd.script_exec.current_script_id.clone().unwrap())
    }

    #[pyfunction]
    fn scene(vm: &VirtualMachine) -> PyObjectRef {
        if let Some(scene) = with_gd(|gd| gd.script_exec.scene.clone()) {
            scene.into_pyobject(vm)
        } else {
            PyNone.into_object(vm)
        }
    }

    #[pyfunction]
    fn set_gvar(name: PyStrRef, value: PyObjectRef, vm: &VirtualMachine) -> PyResult<()> {
        let value = py_to_value(vm, value)?;
        with_gd_mut(|gd| gd.vars.set_global_var(name.as_ref(), value));
        Ok(())
    }

    #[pyfunction]
    fn exist_gvar(name: PyStrRef) -> bool {
        with_gd(|gd| gd.vars.global_var(name.as_ref()).is_some())
    }

    #[pyfunction]
    fn get_gvar(name: PyStrRef, vm: &VirtualMachine) -> PyObjectRef {
        if let Some(value) = with_gd(|gd| gd.vars.global_var(name.as_ref()).cloned()) {
            value_to_py(vm, value)
        } else {
            PyNone.into_object(vm)
        }
    }

    #[pyfunction]
    fn get_var(name: PyStrRef, vm: &VirtualMachine) -> PyObjectRef {
        if let Some(value) = with_gd(|gd| {
            gd.vars
                .local_var(
                    gd.script_exec.current_script_id.as_ref().unwrap(),
                    name.as_ref(),
                )
                .cloned()
        }) {
            value_to_py(vm, value)
        } else {
            PyNone.into_object(vm)
        }
    }

    #[pyfunction]
    fn set_var(name: PyStrRef, value: PyObjectRef, vm: &VirtualMachine) -> PyResult<()> {
        let value = py_to_value(vm, value)?;
        with_gd_mut(|gd| {
            gd.vars.set_local_var(
                gd.script_exec.current_script_id.as_ref().unwrap(),
                name.as_ref(),
                value,
            )
        });
        Ok(())
    }

    #[pyfunction]
    fn exist_var(name: PyStrRef) -> bool {
        with_gd(|gd| {
            gd.vars
                .local_var(
                    gd.script_exec.current_script_id.as_ref().unwrap(),
                    name.as_ref(),
                )
                .is_some()
        })
    }

    #[pyfunction]
    fn current_time() -> i64 {
        with_gd(|gd| gd.time.current_time().as_secs().try_into().unwrap())
    }

    #[pyfunction]
    fn number_of_dead_party_members() -> usize {
        with_gd(|gd| gd.player.party_dead.len())
    }

    #[pyfunction]
    fn has_empty_for_party() -> bool {
        with_gd(|gd| call_game_method!(has_empty_for_party)(gd))
    }

    #[pyfunction]
    fn has_item(id: PyStrRef) -> u32 {
        with_gd(|gd| call_game_method!(has_item)(gd, id.as_ref()).unwrap_or(0))
    }

    #[pyfunction]
    fn gen_dungeons() {
        with_gd_mut(|gd| call_game_method!(gen_dungeons)(gd))
    }

    #[pyfunction]
    fn gen_party_chara(id: PyStrRef, lv: u32) -> bool {
        with_gd_mut(|gd| call_game_method!(gen_party_chara)(gd, id.as_ref(), lv))
    }

    #[pyfunction]
    fn receive_quest_rewards() -> bool {
        with_gd_mut(|gd| call_game_method!(receive_quest_rewards)(gd))
    }

    #[pyfunction]
    fn receive_item(id: PyStrRef, n: u32) {
        with_gd_mut(|gd| call_game_method!(receive_item)(gd, id.as_ref(), n))
    }

    #[pyfunction]
    fn receive_money(amount: u32) {
        with_gd_mut(|gd| call_game_method!(receive_money)(gd, amount))
    }

    #[pyfunction]
    fn remove_item(id: PyStrRef, n: u32, vm: &VirtualMachine) -> PyResult<()> {
        with_gd_mut(|gd| {
            call_game_method!(remove_item)(gd, id.as_ref(), n)
                .map_err(|_| vm.new_value_error("remove item failed".into()))?;
            Ok(())
        })
    }

    #[pyfunction]
    fn resurrect_party_members() {
        with_gd_mut(|gd| {
            call_game_method!(resurrect_party_members)(gd);
        })
    }
}
