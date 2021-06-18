use crate::basic::SAVE_EXTENSION;
use crate::gamedata::*;
use crate::impl_filebox::MapLoadError;
use crate::utils::to_writer_with_mode;
use serde_cbor::from_reader;
use std::fs::{self, create_dir_all, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

#[cfg(feature = "global_state_obj")]
impl GameData {
    /// Save game data to the specified directory
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        if cfg!(debug_assertions) {
            print_save_data_size(self); // Debug code for save file size optimization
        }

        let save_dir = path.as_ref();

        // Create directory
        create_dir_all(&save_dir)?;

        // Write id table file
        let mut file = BufWriter::new(File::create(save_dir.join("idtable"))?);
        writeln!(file, "{:016x}", *crate::gobj::OBJ_HOLDER_HASH)?;
        crate::gobj::get_objholder().write_table(&mut file)?;

        // Write metadata file
        let mut file = BufWriter::new(File::create(save_dir.join("metadata"))?);
        serde_json::to_writer_pretty(&mut file, &self.meta)?;

        // Write GameData
        let mut file = BufWriter::new(File::create(save_dir.join("gamedata"))?);
        to_writer_with_mode(&mut file, &self)?;

        // Write maps
        let map_dir = save_dir.join("maps");
        create_dir_all(&map_dir)?;

        let mut errors: Vec<MapLoadError> = Vec::new();
        self.region
            .visit_all_maps(|_mid, map| match BoxedMap::write(map, &map_dir) {
                Ok(_) => (),
                Err(e) => errors.push(e),
            });

        if !errors.is_empty() {
            return Err(errors.into_iter().next().unwrap().into());
        }

        Ok(())
    }

    /// Load game data from specified directory
    pub fn load<P: AsRef<Path>>(path: P) -> Result<GameData, Box<dyn std::error::Error>> {
        let save_dir = path.as_ref();

        // Read metadata file
        let mut file = BufReader::new(File::open(save_dir.join("metadata"))?);
        let meta: MetaData = serde_json::from_reader(&mut file)?;

        // Read index conversion table
        let mut file = BufReader::new(File::open(save_dir.join("idtable"))?);
        let idx_conv_table =
            crate::idx_conv::IdxConvTable::read(&mut file, *crate::gobj::OBJ_HOLDER_HASH)?;
        let is_table_changed = idx_conv_table.is_some();
        if is_table_changed {
            info!("Detected changes in the id table. Conversion table is created.");
        }
        crate::idx_conv::set_idx_conv_table(idx_conv_table);

        // Read GameData
        let mut file = BufReader::new(File::open(save_dir.join("gamedata"))?);
        let mut gamedata: GameData = from_reader(&mut file)?;
        gamedata.meta = meta;

        let map_dir = save_dir.join("maps");
        if is_table_changed {
            // Preload is needed if id table is changed
            let mut mid_vec = Vec::new();
            gamedata.region.visit_all_maps(|mid, _map| {
                mid_vec.push(mid);
            });
            for mid in &mid_vec {
                gamedata.region.preload_map(*mid, &map_dir);
            }
        } else {
            // Preload current map
            let mid = gamedata.get_current_mapid();
            gamedata.region.preload_map(mid, &map_dir);
        }

        Ok(gamedata)
    }

    pub fn clean_map_dir<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let map_dir = path.as_ref().join("maps");

        let mut map_files: Vec<PathBuf> = Vec::new();
        for entry in fs::read_dir(&map_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                map_files.push(path);
            }
        }

        let mut map_file_path: Vec<PathBuf> = Vec::new();
        self.region.visit_all_maps(|_mid, map| {
            map_file_path.push(map.path(map_dir.clone()));
        });

        let n_exist_file = map_files.len();
        map_files.retain(|p| !map_file_path.iter().any(|a| a == p));

        if n_exist_file == map_files.len() {
            return Ok(());
        }

        for p in &map_files {
            trace!("Remove unused map file {}", p.to_string_lossy());
            fs::remove_file(p)?;
        }

        Ok(())
    }

    pub fn save_dir<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        path.as_ref()
            .join(format!("{}.{}", self.meta.save_name(), SAVE_EXTENSION))
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
