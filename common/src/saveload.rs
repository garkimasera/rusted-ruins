
use std::path::Path;
use std::fs::File;
use serde_cbor::ser::to_writer_packed;
use serde_cbor::from_reader;
use gamedata::*;

impl GameData {
    pub fn save_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ()> {
        if cfg!(debug_assertions) {
            print_save_data_size(self); // Debug code for save file size optimization
        }
        
        let mut file = File::create(path).unwrap();
        to_writer_packed(&mut file, &self).unwrap();

        Ok(())
    }

    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<GameData, ()> {
        let file = File::open(path).unwrap();

        Ok(from_reader(&file).unwrap())
    }
}

/// Print save data size
#[cfg(debug_assertions)]
fn print_save_data_size(gd: &GameData) {
    use serde_cbor::ser::to_vec_packed;
    let v = to_vec_packed(&gd).unwrap();
    println!("Total size = {}", v.len());
    let v = to_vec_packed(&gd.region.0).unwrap();
    println!("Regions size = {}", v.len());
    let v = to_vec_packed(&gd.region.get(RegionId::default())).unwrap();
    println!("Region size = {}", v.len());
    let map = gd.get_current_map();
    let v = to_vec_packed(&map).unwrap();
    println!("Current map size = {}", v.len());
    let v = to_vec_packed(&map.tile).unwrap();
    println!("Current map tiles size = {}", v.len());
}

#[cfg(not(debug_assertions))]
fn print_save_data_size(_gd: &GameData) {}
