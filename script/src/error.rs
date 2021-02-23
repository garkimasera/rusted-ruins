use rustpython_vm::{
    compile::CompileError,
    exceptions::{write_exception, PyBaseExceptionRef},
    VirtualMachine,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("python exception:\n{0}")]
    Python(String),
    #[error("python compile failed:\n{0}")]
    Compile(#[from] CompileError),
    #[error("{0}")]
    JsonSerde(#[from] serde_json::error::Error),
}

impl Error {
    pub fn from_py(vm: &VirtualMachine, e: PyBaseExceptionRef) -> Self {
        let mut buf = Vec::new();
        let _ = write_exception(&mut buf, vm, &e);
        let s = String::from_utf8_lossy(&buf).into_owned();
        Error::Python(s)
    }
}
