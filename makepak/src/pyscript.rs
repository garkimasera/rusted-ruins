use crate::verbose::print_verbose;
use anyhow::*;
use common::obj::{Object, ScriptObject};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::Path;

static FIRST_LINE: Lazy<Regex> = Lazy::new(|| Regex::new("# rusted-ruins-script").unwrap());
static ID_LINE: Lazy<Regex> =
    Lazy::new(|| Regex::new("# id = \"([a-zA-Z][a-zA-Z0-9_.-]*)\"").unwrap());

/// Read python script file
pub fn read_pyscript<P: AsRef<Path>>(path: P) -> Result<Object> {
    let path = path.as_ref();
    let mut f = BufReader::new(File::open(path)?);
    print_verbose(|| format!("Processing \"{:?}\"", path));

    // Check the first line
    let mut first_line = String::new();
    f.read_line(&mut first_line)?;
    if !FIRST_LINE.is_match(&first_line) {
        bail!(
            "the first line of {} is not vaild. Must start with \"# rusted-ruins-script\"",
            path.to_string_lossy()
        );
    }

    // Check the second line for id
    let mut second_line = String::new();
    f.read_line(&mut second_line)?;
    let id = if let Some(caps) = ID_LINE.captures(&second_line) {
        caps.get(1).unwrap().as_str().to_owned()
    } else {
        if let Some(file_stem) = path
            .file_stem()
            .and_then(|file_stem| file_stem.to_os_string().into_string().ok())
        {
            file_stem
        } else {
            bail!(
                "file name of \"{}\" cannot be use for object id",
                path.to_string_lossy()
            );
        }
    };

    f.seek(SeekFrom::Start(0))?;
    let mut script = String::new();
    f.read_to_string(&mut script)?;

    Ok(Object::Script(ScriptObject { id, script }))
}
