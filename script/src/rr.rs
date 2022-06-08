//! Python module for game script

use common::gamedata::Value;
use rustpython_vm::{
    builtins::{PyInt, PyNone, PyStr},
    convert::ToPyObject,
    pymodule, VirtualMachine,
    {object::PyPayload, PyObjectRef, PyResult},
};

pub(crate) use _rr::{make_module, PyGame};

trait ValueExt: Sized {
    fn to_py(self, vm: &VirtualMachine) -> PyObjectRef;
    fn to_py_opt(self, vm: &VirtualMachine) -> Option<PyObjectRef>;
    fn from_py(vm: &VirtualMachine, pyvalue: PyObjectRef) -> PyResult<Self>;
}

impl ValueExt for Value {
    fn to_py(self, vm: &VirtualMachine) -> PyObjectRef {
        match self {
            Value::None => PyNone.into_pyobject(vm),
            Value::Bool(value) => value.to_pyobject(vm),
            Value::Int(value) => value.to_pyobject(vm),
            Value::String(value) => value.to_pyobject(vm),
        }
    }

    fn to_py_opt(self, vm: &VirtualMachine) -> Option<PyObjectRef> {
        match self {
            Value::None => None,
            _ => Some(self.to_py(vm)),
        }
    }

    fn from_py(vm: &VirtualMachine, pyvalue: PyObjectRef) -> PyResult<Self> {
        let value = if pyvalue.payload::<PyNone>().is_some() {
            Value::None
        } else if let Some(i) = pyvalue.payload::<PyInt>() {
            let i: i64 = i.as_bigint().try_into().expect("Failed bigint conversion");
            Value::Int(i)
        } else if let Some(s) = pyvalue.payload::<PyStr>() {
            Value::String(s.as_str().to_owned())
        } else {
            return Err(
                vm.new_type_error(format!("Invalid type value \"{}\" for vars/gvars", pyvalue))
            );
        };
        Ok(value)
    }
}

#[pymodule(name = "rr")]
mod _rr {
    use super::ValueExt;
    use crate::message::ScriptMessage;
    use crate::{GameMethod, TalkText, UiRequest};
    use common::gamedata::{GameData, SkillKind, Value};
    use rustpython_vm::builtins::PyIntRef;
    use rustpython_vm::{
        builtins::{PyListRef, PyStrRef},
        pyclass, pyimpl, FromArgs, PyObjectRef, PyPayload, PyResult, VirtualMachine,
    };
    use std::str::FromStr;

    #[pyattr(name = "Game")]
    #[pyclass(module = "rr", name = "Game")]
    #[derive(Clone, Debug, PyPayload)]
    pub(crate) struct PyGame {
        pub args: std::collections::HashMap<String, Value>,
        pub self_id: String,
        pub method_tx: crossbeam_channel::Sender<ScriptMessage>,
        pub method_result_rx: crossbeam_channel::Receiver<Value>,
    }

    impl PyGame {
        fn send_message(&self, msg: ScriptMessage) -> Value {
            self.method_tx.send(msg).unwrap_or_else(|e| {
                log::trace!("{}", e);
                std::thread::sleep(std::time::Duration::from_secs(60));
                panic!()
            });
            self.method_result_rx.recv().unwrap_or_else(|e| {
                log::trace!("{}", e);
                std::thread::sleep(std::time::Duration::from_secs(60));
                panic!()
            })
        }

        fn call_method(&self, method: GameMethod) -> Value {
            self.send_message(ScriptMessage::Method(method))
        }

        fn with_gd<F: FnOnce(&mut GameData) -> Value + Send + 'static>(&self, f: F) -> Value {
            self.send_message(ScriptMessage::Exec(Box::new(f)))
        }
    }

    #[pyimpl]
    impl PyGame {
        #[pymethod]
        fn self_id(&self) -> String {
            self.self_id.clone()
        }

        #[pyproperty]
        fn args(&self) -> PyScriptArgs {
            PyScriptArgs(self.clone())
        }

        #[pyproperty]
        fn gvars(&self) -> PyGvars {
            PyGvars(self.clone())
        }

        #[pyproperty]
        fn vars(&self) -> PyVars {
            PyVars(self.clone())
        }

        #[pymethod]
        fn current_time(&self, vm: &VirtualMachine) -> PyObjectRef {
            self.with_gd(|gd| Value::Int(gd.time.current_time().as_secs() as _))
                .to_py(vm)
        }

        #[pymethod]
        fn number_of_dead_party_members(&self, vm: &VirtualMachine) -> PyObjectRef {
            self.with_gd(|gd| Value::Int(gd.player.party_dead.len() as _))
                .to_py(vm)
        }

        #[pymethod]
        fn custom_quest_completed(&self, id: PyStrRef, vm: &VirtualMachine) -> PyObjectRef {
            let id = id.as_str().to_owned();
            self.with_gd(move |gd| Value::Bool(gd.quest.completed_custom_quests.contains(&id)))
                .to_py(vm)
        }

        // Ui request methods

        #[pymethod]
        fn talk(&self, opts: TalkOptions, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            let text_id = opts.text_id.as_str().to_owned();
            let choices = if let Some(list) = opts.choices {
                list.borrow_vec()
                    .iter()
                    .map(|pyvalue| Ok(pyvalue.str(vm)?.as_str().to_owned()))
                    .collect::<Result<Vec<String>, _>>()?
            } else {
                Vec::new()
            };
            let target_chara = opts.target_chara.map(|s| s.as_str().to_owned());

            let response = self.send_message(ScriptMessage::UiRequest(UiRequest::Talk {
                talk: TalkText {
                    text_id,
                    choices,
                    target_chara,
                },
            }));
            Ok(response.to_py(vm))
        }

        #[pymethod]
        fn shop_buy(&self) {
            self.send_message(ScriptMessage::UiRequest(UiRequest::ShopBuy));
        }

        #[pymethod]
        fn shop_sell(&self) {
            self.send_message(ScriptMessage::UiRequest(UiRequest::ShopSell));
        }

        #[pymethod]
        fn quest_offer(&self) {
            self.send_message(ScriptMessage::UiRequest(UiRequest::QuestOffer));
        }

        #[pymethod]
        fn quest_report(&self) {
            self.send_message(ScriptMessage::UiRequest(UiRequest::QuestReport));
        }

        #[pymethod]
        fn install_ability_slot(&self) {
            self.send_message(ScriptMessage::UiRequest(UiRequest::InstallAbilitySlot));
        }

        #[pymethod]
        fn install_extend_slot(&self) {
            self.send_message(ScriptMessage::UiRequest(UiRequest::InstallExtendSlot));
        }

        // ScriptMethod methods

        #[pymethod]
        fn complete_custom_quest(&self, id: PyStrRef, vm: &VirtualMachine) -> PyObjectRef {
            self.call_method(GameMethod::CompleteCustomQuest {
                id: id.as_str().to_owned(),
            })
            .to_py(vm)
        }

        #[pymethod]
        fn custom_quest_started(&self, id: PyStrRef, vm: &VirtualMachine) -> PyObjectRef {
            self.call_method(GameMethod::CustomQuestStarted {
                id: id.as_str().to_owned(),
            })
            .to_py(vm)
        }

        #[pymethod]
        fn gen_dungeons(&self) {
            self.call_method(GameMethod::GenDungeons);
        }

        #[pymethod]
        fn gen_party_chara(&self, id: PyStrRef, lv: PyIntRef) {
            self.call_method(GameMethod::GenPartyChara {
                id: id.as_str().to_owned(),
                lv: lv.as_bigint().try_into().unwrap_or(1),
            });
        }

        #[pymethod]
        fn has_empty_for_party(&self, vm: &VirtualMachine) -> PyObjectRef {
            self.call_method(GameMethod::HasEmptyForParty).to_py(vm)
        }

        #[pymethod]
        fn number_of_item(&self, id: PyStrRef, vm: &VirtualMachine) -> PyObjectRef {
            self.call_method(GameMethod::NumberOfItem {
                id: id.as_str().to_owned(),
            })
            .to_py(vm)
        }

        #[pymethod]
        fn receive_item(&self, id: PyStrRef, n: PyIntRef) {
            self.call_method(GameMethod::ReceiveItem {
                id: id.as_str().to_owned(),
                n: n.as_bigint().try_into().unwrap_or(1),
            });
        }

        #[pymethod]
        fn receive_money(&self, amount: PyIntRef) {
            self.call_method(GameMethod::ReceiveMoney {
                amount: amount.as_bigint().try_into().unwrap_or(0),
            });
        }

        #[pymethod]
        fn remove_item(&self, id: PyStrRef, n: PyIntRef) {
            self.call_method(GameMethod::RemoveItem {
                id: id.as_str().to_owned(),
                n: n.as_bigint().try_into().unwrap_or(0),
            });
        }

        #[pymethod]
        fn resurrect_party_members(&self) {
            self.call_method(GameMethod::ResurrectPartyMembers);
        }

        #[pymethod]
        fn start_custom_quest(&self, id: PyStrRef, phase: PyStrRef) {
            self.call_method(GameMethod::StartCustomQuest {
                id: id.as_str().to_owned(),
                phase: phase.as_str().to_owned(),
            });
        }

        #[pymethod]
        fn skill_level(&self, skill: PyStrRef, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            let skill_kind = SkillKind::from_str(skill.as_str())
                .map_err(|e| vm.new_value_error(e.to_string()))?;
            Ok(self
                .call_method(GameMethod::SkillLevel { skill_kind })
                .to_py(vm))
        }

        #[pymethod]
        fn learn_skill(&self, skill: PyStrRef, vm: &VirtualMachine) -> PyResult<()> {
            let skill_kind = SkillKind::from_str(skill.as_str())
                .map_err(|e| vm.new_value_error(e.to_string()))?;
            self.call_method(GameMethod::LearnSkill { skill_kind });
            Ok(())
        }
    }

    #[derive(FromArgs)]
    struct TalkOptions {
        #[pyarg(positional)]
        text_id: PyStrRef,
        #[pyarg(any, optional)]
        choices: Option<PyListRef>,
        #[pyarg(any, optional)]
        target_chara: Option<PyStrRef>,
    }

    #[pyattr(name = "ScriptArgs")]
    #[pyclass(module = "rr", name = "ScriptArgs")]
    #[derive(Debug, PyPayload)]
    pub(crate) struct PyScriptArgs(PyGame);

    #[pyimpl]
    impl PyScriptArgs {
        #[pymethod(magic)]
        fn getitem(&self, key: PyStrRef, vm: &VirtualMachine) -> Option<PyObjectRef> {
            self.0
                .args
                .get(key.as_str())
                .and_then(|value| value.clone().to_py_opt(vm))
        }
    }

    #[pyattr(name = "Gvars")]
    #[pyclass(module = "rr", name = "Gvars")]
    #[derive(Debug, PyPayload)]
    pub(crate) struct PyGvars(PyGame);

    #[pyimpl]
    impl PyGvars {
        #[pymethod(magic)]
        fn getitem(&self, key: PyStrRef, vm: &VirtualMachine) -> Option<PyObjectRef> {
            self.0
                .with_gd(move |gd| {
                    gd.vars
                        .global_var(key.as_str())
                        .cloned()
                        .unwrap_or(Value::None)
                })
                .to_py_opt(vm)
        }

        #[pymethod(magic)]
        fn setitem(&self, key: PyStrRef, value: PyObjectRef, vm: &VirtualMachine) -> PyResult<()> {
            let value = Value::from_py(vm, value)?;
            self.0.with_gd(move |gd| {
                gd.vars.set_global_var(key.as_str(), value);
                Value::None
            });
            Ok(())
        }

        #[pymethod(magic)]
        fn delitem(&self, key: PyStrRef) -> PyResult<()> {
            self.0.with_gd(move |gd| {
                gd.vars.remove_global_var(key.as_str());
                Value::None
            });
            Ok(())
        }
    }

    #[pyattr(name = "Vars")]
    #[pyclass(module = "rr", name = "Vars")]
    #[derive(Debug, PyPayload)]
    pub(crate) struct PyVars(PyGame);

    #[pyimpl]
    impl PyVars {
        #[pymethod(magic)]
        fn getitem(&self, key: PyStrRef, vm: &VirtualMachine) -> Option<PyObjectRef> {
            let self_id = self.0.self_id.clone();
            self.0
                .with_gd(move |gd| {
                    gd.vars
                        .local_var(&self_id, key.as_str())
                        .cloned()
                        .unwrap_or(Value::None)
                })
                .to_py_opt(vm)
        }

        #[pymethod(magic)]
        fn setitem(&self, name: PyStrRef, value: PyObjectRef, vm: &VirtualMachine) -> PyResult<()> {
            let value = Value::from_py(vm, value)?;
            let self_id = self.0.self_id.clone();
            let name = name.as_str().to_owned();
            self.0.with_gd(move |gd| {
                gd.vars.set_local_var(self_id, name, value);
                Value::None
            });
            Ok(())
        }

        #[pymethod(magic)]
        fn delitem(&self, key: PyStrRef) -> PyResult<()> {
            let self_id = self.0.self_id.clone();
            self.0.with_gd(move |gd| {
                gd.vars.remove_local_var(&self_id, key.as_str());
                Value::None
            });
            Ok(())
        }
    }
}
