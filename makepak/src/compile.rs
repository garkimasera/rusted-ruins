use crate::dir;
use crate::error::*;
use crate::pyscript::read_pyscript;
use crate::verbose::print_verbose;
use anyhow::{bail, Result};
use common::obj::Object;
use common::pakutil::write_object;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use crate::buildobj::build_object;
use crate::input::Input;

pub fn compile(files: &[&str], output_file: &str) {
    let out = File::create(output_file).unwrap();
    let mut builder = tar::Builder::new(out);

    for f in files {
        let f = Path::new(f);
        if f.is_relative() {
            dir::set_src_dir(f.parent());
        } else {
            dir::set_src_dir(None);
        }

        let read_result = if Some(true) == f.extension().map(|e| e == "py") {
            read_pyscript(f)
        } else {
            read_input_file(f)
        };

        let obj = match read_result {
            Ok(o) => o,
            Err(e) => {
                eprintln!("Cannot process \"{}\"", f.to_string_lossy());
                for e in e.chain() {
                    eprintln!("{}", e);
                }
                continue;
            }
        };
        let v = write_to_vec(&obj).unwrap();
        write_data_to_tar(&mut builder, &v, obj.get_id());
    }
    builder.finish().unwrap();
}

fn read_input_file<P: AsRef<Path>>(path: P) -> Result<Object> {
    let path = path.as_ref();
    let s = {
        let mut f = File::open(path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        s
    };

    print_verbose(|| format!("Processing \"{:?}\"", path));

    let ext = if let Some(ext) = path.extension() {
        ext
    } else {
        bail!(
            "given file does not have extension: {}",
            path.to_string_lossy()
        );
    };

    let input: Input = if ext == "ron" {
        ron::de::from_str(&s)?
    } else {
        bail!("invalid input file type: {}", path.to_string_lossy());
    };

    print_verbose(|| format!("{:?}", input));
    let object = build_object(input)?;

    Ok(object)
}

fn write_to_vec(obj: &Object) -> Result<Vec<u8>> {
    let mut v = Vec::new();
    match write_object(&mut v, obj) {
        Ok(_) => Ok(v),
        Err(e) => bail!(PakCompileError::ObjWriteError { description: e }),
    }
}

fn write_data_to_tar<W: Write>(builder: &mut tar::Builder<W>, data: &[u8], path: &str) {
    let mut header = tar::Header::new_gnu();
    header.set_path(path).unwrap();
    header.set_size(data.len() as u64);
    header.set_mtime(0);
    header.set_cksum();

    builder.append(&header, data).unwrap();
}
