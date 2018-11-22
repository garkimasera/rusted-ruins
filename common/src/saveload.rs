
use std::path::Path;
use std::fs::File;
use serde_cbor::ser::to_writer_packed;
use serde_cbor::from_reader;
use gamedata::GameData;

impl GameData {
    pub fn save_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ()> {
        let mut file = File::create(path).unwrap();

        to_writer_packed(&mut file, &self).unwrap();
        Ok(())
    }

    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<GameData, ()> {
        let file = File::open(path).unwrap();

        Ok(from_reader(&file).unwrap())
    }
}

