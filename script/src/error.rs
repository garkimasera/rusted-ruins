use rustpython_vm::{builtins::PyBaseExceptionRef, compiler::CompileError, VirtualMachine};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("python exception:\n{0}")]
    Python(String),
    #[error("python compile failed:\n{0}")]
    Compile(#[from] CompileError),
    #[error("object not found")]
    NoObject(String),
}

impl Error {
    pub fn from_py(vm: &VirtualMachine, e: PyBaseExceptionRef) -> Self {
        let mut s = String::new();
        let _ = vm.write_exception(&mut s, &e);
        Error::Python(s)
    }
}
