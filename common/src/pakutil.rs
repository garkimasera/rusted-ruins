
use std::io::{Read, Write};
use rmps::encode::write;
use rmps::decode::from_read;

use super::obj::Object;

/// Read object from msgpack bytes
pub fn read_object<R: Read>(r: R) -> Result<Object, ::rmps::decode::Error> {
    from_read(r)
}

/// Write object as msgpack
pub fn write_object<W: Write>(w: &mut W, obj: &Object) -> Result<(), String> {
    write(w, obj).map_err(|e| e.to_string())
}

/*
  Implement load_objs_dir
*/
use std::path::Path;
use std::fs;
use tar;

pub enum PakLoadingError {
    Io(::std::io::Error),
    Rmps(::rmps::decode::Error),
}


/// Load objects from pak files recursively
pub fn load_objs_dir<F: FnMut(Object)>(dir: &Path, cb: F) -> Vec<PakLoadingError> {
    let mut err_stack = Vec::new();
    let mut cb = cb;

    walk_dir(dir, &mut cb, &mut err_stack);
    err_stack
}

fn walk_dir(dir: &Path, cb: &mut FnMut(Object), err_stack: &mut Vec<PakLoadingError>) {
    let entry_iter = match fs::read_dir(dir) {
        Ok(o) => o,
        Err(e) => {
            err_stack.push(PakLoadingError::Io(e));
            return;
        },
    };

    for entry in entry_iter {
        let entry = match entry {
            Ok(o) => o,
            Err(e) => {
                err_stack.push(PakLoadingError::Io(e));
                continue;
            },
        };
        let path = entry.path();
        if path.is_dir() {
            walk_dir(&path, cb, err_stack);
        } else if path.extension() != None && path.extension().unwrap() == "pak" {
            read_tar(&path, cb, err_stack);
        }
    }
}

/// Read tar file and load objects
pub fn read_tar(path: &Path, cb: &mut FnMut(Object), err_stack: &mut Vec<PakLoadingError>) {
    let outputfile = match fs::File::open(path) {
        Ok(o) => o,
        Err(e) => {
            err_stack.push(PakLoadingError::Io(e));
            return;
        }
    };

    let mut ar = tar::Archive::new(outputfile);

    let entries = match ar.entries() {
        Ok(o) => o,
        Err(e) => {
            err_stack.push(PakLoadingError::Io(e));
            return;
        }
    };

    for file in entries {
        let file = match file {
            Ok(o) => o,
            Err(e) => {
                err_stack.push(PakLoadingError::Io(e));
                continue;
            }
        };
        
        let object = match read_object(file) {
            Ok(o) => o,
            Err(e) => {
                err_stack.push(PakLoadingError::Rmps(e));
                continue;
            }
        };

        cb(object);
    }
}


