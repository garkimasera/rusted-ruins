use anyhow::Error;
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tar::Archive;

pub fn datwalker<F: FnMut(String, &'static [u8])>(
    path: &Path,
    target_ext: &str,
    mut f: F,
) -> Result<(), Error> {
    let file = File::open(path)?;
    let mut a = Archive::new(GzDecoder::new(file));

    let entries = a.entries()?;

    for file in entries {
        let mut file = warn_continue!(file);
        let path = warn_continue!(file.header().path());

        if path.extension().is_some() && path.extension().unwrap() == target_ext {
            let filename = warn_continue!(path.file_stem().ok_or("Invalid file name"))
                .to_string_lossy()
                .into_owned();

            let mut buf = Vec::new();
            warn_continue!(file.read_to_end(&mut buf));
            let data: &'static [u8] = Box::leak(buf.into());
            f(filename, data);
        }
    }

    Ok(())
}
