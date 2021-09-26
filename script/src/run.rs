use crate::Error;
use crate::ScriptYield;
use rustpython_vm as vm;
use vm::scope::Scope;

pub fn run(vm: &vm::VirtualMachine, scope: &Scope) -> Result<Option<ScriptYield>, Error> {
    let code_obj = vm.compile(
        "_get_next_script_yield()",
        vm::compile::Mode::Eval,
        "<get_next>".into(),
    )?;

    match vm.run_code_obj(code_obj, scope.clone()) {
        Ok(output) => {
            if vm.is_none(&output) {
                return Ok(None);
            }
            let mut buf = Vec::new();
            let mut serializer = serde_json::ser::Serializer::new(&mut buf);
            vm::py_serde::serialize(vm, &output, &mut serializer)?;
            let s = String::from_utf8_lossy(&buf);
            let result: ScriptYield = serde_json::from_str(&s)?;

            Ok(Some(result))
        }
        Err(e) => Err(Error::from_py(vm, e)),
    }
}
