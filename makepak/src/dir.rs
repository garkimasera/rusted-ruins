use std::cell::Cell;
use std::env::current_dir;
use std::path::{Path, PathBuf};

thread_local!(
    pub static SRC_DIR: Cell<Option<PathBuf>> = Cell::new(None);
);

pub fn set_src_dir(path: Option<&Path>) {
    let path = path.to_owned();

    SRC_DIR.with(|src_dir| {
        src_dir.replace(path.map(|p| p.to_owned()));
    });
}

pub fn get_src_dir() -> PathBuf {
    SRC_DIR.with(|src_dir| {
        let tmp = src_dir.replace(None);
        let return_val: PathBuf = if let Some(ref path) = tmp {
            path.clone()
        } else {
            current_dir().expect("Cannot get current directory")
        };
        src_dir.replace(tmp);
        return_val
    })
}

pub fn path_from_src_dir<P: AsRef<Path>>(p: P) -> PathBuf {
    let p = p.as_ref();
    let mut src_dir = get_src_dir();
    src_dir.push(p);
    src_dir
}
