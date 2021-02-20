use common::obj::MapTemplateObject;
use common::obj::Object;
use common::pakutil;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn load_from_file(path: &Path) -> Result<MapTemplateObject, Box<dyn Error>> {
    let mut mapobj: Option<MapTemplateObject> = None;
    let mut errors = Vec::new();

    pakutil::read_tar(
        path,
        &mut |object| match object {
            Object::MapTemplate(o) => {
                mapobj = Some(o);
            }
            _ => (),
        },
        &mut errors,
    );

    Ok(mapobj.ok_or("Object is not found")?)
}

pub fn save_to_file(path: &Path, map: MapTemplateObject) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;
    let mut builder = tar::Builder::new(file);
    let obj = Object::MapTemplate(map);
    let mut data: Vec<u8> = Vec::new();
    pakutil::write_object(&mut data, &obj).unwrap();
    write_data_to_tar(&mut builder, &data, obj.get_id());
    builder.finish()?;
    Ok(())
}

fn write_data_to_tar<W: Write>(builder: &mut tar::Builder<W>, data: &[u8], path: &str) {
    let mut header = tar::Header::new_gnu();
    header.set_path(path).unwrap();
    header.set_size(data.len() as u64);
    header.set_mtime(get_unix_time());
    header.set_cksum();

    builder.append(&header, data).unwrap();
}

use std::time::{SystemTime, UNIX_EPOCH};

fn get_unix_time() -> u64 {
    let duration = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d,
        Err(_) => {
            return 0;
        }
    };
    duration.as_secs()
}
