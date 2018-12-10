
use std::path::{Path, PathBuf};
use std::fs::{File, create_dir_all};
use std::io::BufWriter;
use serde_cbor::ser::to_writer_packed;
use serde_cbor::from_reader;
use basic::SAVE_EXTENSION;
use gamedata::*;
use impl_filebox::MapLoadError;

impl GameData {
    /// Save game data to the specified directory
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<::std::error::Error>> {
        if cfg!(debug_assertions) {
            print_save_data_size(self); // Debug code for save file size optimization
        }

        let save_dir = path.as_ref();

        // Create directory
        create_dir_all(&save_dir)?;

        // Write metadata file
        let mut file = BufWriter::new(File::create(save_dir.join("metadata"))?);
        serde_json::to_writer_pretty(&mut file, &self.meta)?;

        // Write GameData
        let mut file = BufWriter::new(File::create(save_dir.join("gamedata"))?);
        to_writer_packed(&mut file, &self)?;

        // Write maps
        let map_dir = save_dir.join("maps");
        create_dir_all(&map_dir)?;
        
        let mut errors: Vec<MapLoadError> = Vec::new();
        self.region.visit_all_maps(|_mid, map| {
            match BoxedMap::write(map, &map_dir) {
                Ok(_) => (),
                Err(e) => errors.push(e),
            }
        });

        if !errors.is_empty() {
            return Err(errors.into_iter().next().unwrap().into());
        }

        Ok(())
    }

    /// Load game data from specified directory
    pub fn load<P: AsRef<Path>>(path: P) -> Result<GameData, String> {
        let file = File::open(path).map_err(|e| e.to_string())?;

        from_reader(&file).map_err(|e| e.to_string())
    }

    pub fn save_dir<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        path.as_ref().join(format!("{}.{}", self.meta.save_name(), SAVE_EXTENSION))
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
