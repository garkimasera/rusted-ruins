//! Python module for game script

use common::gamedata::Value;
use rustpython_vm::{
    builtins::{PyInt, PyNone, PyStr},
    convert::ToPyObject,
    pymodule, VirtualMachine,
    {object::PyPayload, PyObjectRef, PyResult},
};

pub(crate) use _rr::{make_module, PyGame, ScriptMethodErr};

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
                vm.new_type_error(format!("Invalid type value \"{pyvalue:?}\" for vars/gvars"))
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
    use once_cell::sync::Lazy;
    use rustpython_vm::{
        atomic_func,
        builtins::{PyIntRef, PyListRef, PyStrRef},
        convert::ToPyObject,
        protocol::{PyMappingMethods, PySequenceMethods},
        pyclass,
        types::{AsMapping, AsSequence},
        FromArgs, PyObject, PyObjectRef, PyPayload, PyResult, VirtualMachine,
    };
    use std::str::FromStr;

    #[derive(Debug, thiserror::Error)]
    #[error("{0}")]
    pub struct ScriptMethodErr(String);

    #[pyattr(name = "Game")]
    #[pyclass(module = "rr", name = "Game")]
    #[derive(Clone, Debug, PyPayload)]
    pub(crate) struct PyGame {
        pub args: std::collections::HashMap<String, Value>,
        pub self_id: String,
        pub method_tx: crossbeam_channel::Sender<ScriptMessage>,
        pub method_result_rx: crossbeam_channel::Receiver<Result<Value, ScriptMethodErr>>,
    }

    impl PyGame {
        fn send_message(&self, msg: ScriptMessage) -> Result<Value, ScriptMethodErr> {
            self.method_tx.send(msg).unwrap_or_else(|e| {
                log::error!("{}", e);
                std::thread::sleep(std::time::Duration::from_secs(60));
                panic!()
            });
            self.method_result_rx.recv().unwrap_or_else(|e| {
                log::error!("{}", e);
                std::thread::sleep(std::time::Duration::from_secs(60));
                panic!()
            })
        }

        fn call_method(&self, method: GameMethod) -> Value {
            self.send_message(ScriptMessage::Method(method)).unwrap()
        }

        fn with_gd<F: FnOnce(&mut GameData) -> Result<Value, ScriptMethodErr> + Send + 'static>(
            &self,
            f: F,
        ) -> Result<Value, ScriptMethodErr> {
            self.send_message(ScriptMessage::Exec(Box::new(f)))
        }
    }

    #[pyclass]
    impl PyGame {
        #[pymethod]
        fn self_id(&self) -> String {
            self.self_id.clone()
        }

        #[pygetset]
        fn args(&self) -> PyScriptArgs {
            PyScriptArgs(self.clone())
        }

        #[pygetset]
        fn gvars(&self) -> PyGvars {
            PyGvars(self.clone())
        }

        #[pygetset]
        fn vars(&self) -> PyVars {
            PyVars(self.clone())
        }

        #[pymethod]
        fn current_time(&self, vm: &VirtualMachine) -> PyObjectRef {
            self.with_gd(|gd| Ok(Value::Int(gd.time.current_time().as_secs() as _)))
                .unwrap()
                .to_py(vm)
        }

        #[pymethod]
        fn number_of_dead_party_members(&self, vm: &VirtualMachine) -> PyObjectRef {
            self.with_gd(|gd| Ok(Value::Int(gd.player.party_dead.len() as _)))
                .unwrap()
                .to_py(vm)
        }

        #[pymethod]
        fn custom_quest_completed(&self, id: PyStrRef, vm: &VirtualMachine) -> PyObjectRef {
            let id = id.as_str().to_owned();
            self.with_gd(move |gd| Ok(Value::Bool(gd.quest.completed_custom_quests.contains(&id))))
                .unwrap()
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
            Ok(response.unwrap().to_py(vm))
        }

        #[pymethod]
        fn shop_buy(&self) {
            let _ = self.send_message(ScriptMessage::UiRequest(UiRequest::ShopBuy));
        }

        #[pymethod]
        fn shop_sell(&self) {
            let _ = self.send_message(ScriptMessage::UiRequest(UiRequest::ShopSell));
        }

        #[pymethod]
        fn quest_offer(&self) {
            let _ = self.send_message(ScriptMessage::UiRequest(UiRequest::QuestOffer));
        }

        #[pymethod]
        fn quest_report(&self) {
            let _ = self.send_message(ScriptMessage::UiRequest(UiRequest::QuestReport));
        }

        #[pymethod]
        fn install_ability_slot(&self) {
            let _ = self.send_message(ScriptMessage::UiRequest(UiRequest::InstallAbilitySlot));
        }

        #[pymethod]
        fn install_extend_slot(&self) {
            let _ = self.send_message(ScriptMessage::UiRequest(UiRequest::InstallExtendSlot));
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

    #[pyclass]
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

    #[pyclass(with(AsMapping, AsSequence))]
    impl PyGvars {
        #[pymethod(magic)]
        fn contains(&self, key: PyObjectRef, vm: &VirtualMachine) -> PyResult<bool> {
            self._contains(&key, vm)
        }

        fn _contains(&self, key: &PyObject, vm: &VirtualMachine) -> PyResult<bool> {
            let key_str: String = key.try_to_value(vm)?;
            self.0
                .with_gd(move |gd| Ok(Value::Bool(gd.vars.global_var(&key_str).is_some())))
                .map(|value| matches!(value, Value::Bool(true)))
                .map_err(|_| unreachable!())
        }

        #[pymethod(magic)]
        fn getitem(&self, key: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            self._getitem(&key, vm)
        }

        fn _getitem(&self, key: &PyObject, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            let key_str: String = key.try_to_value(vm)?;
            self.0
                .with_gd(move |gd| {
                    gd.vars
                        .global_var(&key_str)
                        .cloned()
                        .ok_or_else(|| ScriptMethodErr("".into()))
                })
                .map(|value| value.to_py(vm))
                .map_err(|_| vm.new_key_error(key.to_pyobject(vm)))
        }

        #[pymethod(magic)]
        fn setitem(
            &self,
            name: PyObjectRef,
            value: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<()> {
            self._setitem(&name, value, vm)
        }

        fn _setitem(
            &self,
            name: &PyObject,
            value: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<()> {
            let name: String = name.try_to_value(vm)?;
            let value = Value::from_py(vm, value)?;
            let _ = self.0.with_gd(move |gd| {
                gd.vars.set_global_var(name, value);
                Ok(Value::None)
            });
            Ok(())
        }

        #[pymethod(magic)]
        fn delitem(&self, key: PyObjectRef, vm: &VirtualMachine) -> PyResult<()> {
            self._delitem(&key, vm)
        }

        fn _delitem(&self, key: &PyObject, vm: &VirtualMachine) -> PyResult<()> {
            let key_str: String = key.try_to_value(vm)?;
            self.0
                .with_gd(move |gd| {
                    gd.vars.remove_global_var(&key_str);
                    Ok(Value::None)
                })
                .map(|_| ())
                .map_err(|_| vm.new_key_error(key.to_pyobject(vm)))
        }
    }

    impl AsMapping for PyGvars {
        fn as_mapping() -> &'static PyMappingMethods {
            static AS_MAPPING: Lazy<PyMappingMethods> = Lazy::new(|| PyMappingMethods {
                subscript: atomic_func!(|mapping, needle, vm| {
                    PyGvars::mapping_downcast(mapping)._getitem(needle, vm)
                }),
                ass_subscript: atomic_func!(|mapping, needle, value, vm| {
                    let zelf = PyGvars::mapping_downcast(mapping);
                    if let Some(value) = value {
                        zelf._setitem(needle, value, vm)
                    } else {
                        zelf._delitem(needle, vm)
                    }
                }),
                ..PyMappingMethods::NOT_IMPLEMENTED
            });
            &AS_MAPPING
        }
    }

    impl AsSequence for PyGvars {
        fn as_sequence() -> &'static PySequenceMethods {
            static AS_SEQUENCE: Lazy<PySequenceMethods> = Lazy::new(|| PySequenceMethods {
                contains: atomic_func!(
                    |seq, target, vm| PyGvars::sequence_downcast(seq)._contains(target, vm)
                ),
                ..PySequenceMethods::NOT_IMPLEMENTED
            });
            &AS_SEQUENCE
        }
    }

    #[pyattr(name = "Vars")]
    #[pyclass(module = "rr", name = "Vars")]
    #[derive(Debug, PyPayload)]
    pub(crate) struct PyVars(PyGame);

    #[pyclass(with(AsMapping, AsSequence))]
    impl PyVars {
        #[pymethod(magic)]
        fn contains(&self, key: PyObjectRef, vm: &VirtualMachine) -> PyResult<bool> {
            self._contains(&key, vm)
        }

        fn _contains(&self, key: &PyObject, vm: &VirtualMachine) -> PyResult<bool> {
            let self_id = self.0.self_id.clone();
            let key_str: String = key.try_to_value(vm)?;
            self.0
                .with_gd(move |gd| Ok(Value::Bool(gd.vars.local_var(&self_id, &key_str).is_some())))
                .map(|value| matches!(value, Value::Bool(true)))
                .map_err(|_| unreachable!())
        }

        #[pymethod(magic)]
        fn getitem(&self, key: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            self._getitem(&key, vm)
        }

        fn _getitem(&self, key: &PyObject, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
            let self_id = self.0.self_id.clone();
            let key_str: String = key.try_to_value(vm)?;
            self.0
                .with_gd(move |gd| {
                    gd.vars
                        .local_var(&self_id, &key_str)
                        .cloned()
                        .ok_or_else(|| ScriptMethodErr("".into()))
                })
                .map(|value| value.to_py(vm))
                .map_err(|_| vm.new_key_error(key.to_pyobject(vm)))
        }

        #[pymethod(magic)]
        fn setitem(
            &self,
            name: PyObjectRef,
            value: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<()> {
            self._setitem(&name, value, vm)
        }

        fn _setitem(
            &self,
            name: &PyObject,
            value: PyObjectRef,
            vm: &VirtualMachine,
        ) -> PyResult<()> {
            let value = Value::from_py(vm, value)?;
            let self_id = self.0.self_id.clone();
            let name: String = name.try_to_value(vm)?;
            let _ = self.0.with_gd(move |gd| {
                gd.vars.set_local_var(self_id, name, value);
                Ok(Value::None)
            });
            Ok(())
        }

        #[pymethod(magic)]
        fn delitem(&self, key: PyObjectRef, vm: &VirtualMachine) -> PyResult<()> {
            self._delitem(&key, vm)
        }

        fn _delitem(&self, key: &PyObject, vm: &VirtualMachine) -> PyResult<()> {
            let self_id = self.0.self_id.clone();
            let key_str: String = key.try_to_value(vm)?;
            self.0
                .with_gd(move |gd| {
                    gd.vars.remove_local_var(&self_id, &key_str);
                    Ok(Value::None)
                })
                .map(|_| ())
                .map_err(|_| vm.new_key_error(key.to_pyobject(vm)))
        }
    }

    impl AsMapping for PyVars {
        fn as_mapping() -> &'static PyMappingMethods {
            static AS_MAPPING: Lazy<PyMappingMethods> = Lazy::new(|| PyMappingMethods {
                subscript: atomic_func!(|mapping, needle, vm| {
                    PyGvars::mapping_downcast(mapping)._getitem(needle, vm)
                }),
                ass_subscript: atomic_func!(|mapping, needle, value, vm| {
                    let zelf = PyGvars::mapping_downcast(mapping);
                    if let Some(value) = value {
                        zelf._setitem(needle, value, vm)
                    } else {
                        zelf._delitem(needle, vm)
                    }
                }),
                ..PyMappingMethods::NOT_IMPLEMENTED
            });
            &AS_MAPPING
        }
    }

    impl AsSequence for PyVars {
        fn as_sequence() -> &'static PySequenceMethods {
            static AS_SEQUENCE: Lazy<PySequenceMethods> = Lazy::new(|| PySequenceMethods {
                contains: atomic_func!(
                    |seq, target, vm| PyGvars::sequence_downcast(seq)._contains(target, vm)
                ),
                ..PySequenceMethods::NOT_IMPLEMENTED
            });
            &AS_SEQUENCE
        }
    }
}
