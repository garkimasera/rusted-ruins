use crate::{Error, ScriptYield};
use common::gamedata::GameData;
use rustpython_vm as vm;

#[derive(Clone)]
pub struct ScriptEngine<'a> {
    vm: &'a vm::VirtualMachine,
    scope: Option<vm::scope::Scope>,
}

pub fn enter<F: FnOnce(ScriptEngine) -> R, R>(f: F) -> R {
    vm::Interpreter::default().enter(|vm| {
        let script_engine = ScriptEngine { vm, scope: None };
        f(script_engine)
    })
}

impl<'a> ScriptEngine<'a> {
    pub fn start(&mut self, input: &str, name: &str) -> Result<(), Error> {
        let scope = self.vm.new_scope_with_builtins();

        // Add gamedata functions
        crate::gamedata::add_fns(self.vm, &scope).map_err(|e| Error::from_py(self.vm, e))?;

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
            "_rrscript_gen = rrscript_main()",
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
        let result = crate::gamedata::enter(gd, || crate::run::run(self.vm, scope))?;

        if result.is_none() {
            self.scope = None;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::gamedata::GameData;
    use common::script::Value;

    #[test]
    fn start_test() {
        enter(|mut se| {
            let input = r#"
def rrscript_main():
    a = get_gvar("test0")
    set_gvar("test1", a + 12)
    yield ScriptYield.quest()
    return
"#;
            let mut gd = common::gamedata::GameData::default();
            gd.vars.set_global_var("test0", Value::Int(30));
            se.start(input, "test1").unwrap();
            assert_eq!(Some(ScriptYield::Quest), se.next(&mut gd).unwrap());
            assert_eq!(None, se.next(&mut gd).unwrap());
            assert_eq!(Some(&Value::Int(42)), gd.vars.global_var("test1"));
        });
    }
}
