use crate::error::*;
use std::path::Path;

pub fn read_file_as_string<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    use std::fs::File;
    use std::io::Read;

    let mut f = File::open(path.as_ref())?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
