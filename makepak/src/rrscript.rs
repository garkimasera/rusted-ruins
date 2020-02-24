use crate::buildobj::script_parse;
use crate::verbose::print_verbose;
use anyhow::*;
use common::obj::{Object, ScriptObject};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

/// Read rrscript file
pub fn read_rrscript<P: AsRef<Path>>(path: P) -> Result<Object> {
    let mut f = BufReader::new(File::open(path.as_ref())?);
    let mut first_line = String::new();
    f.read_line(&mut first_line)?;
    let mut script_text = String::new();
    f.read_to_string(&mut script_text)?;

    print_verbose(|| format!("Processing \"{:?}\"", path.as_ref()));

    let object_id = first_line.trim().to_owned();
    let script = script_parse(&script_text)?;

    Ok(Object::Script(ScriptObject {
        id: object_id,
        script,
    }))
}
