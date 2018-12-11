
use std::io::{Read, Write, Error as IoError};
use std::fmt::Display;
use serde_cbor::{from_reader, to_writer, error::Error as SerdeError};
use crate::gamedata::Map;
use filebox::*;

impl WithId for Map {
    type Error = MapLoadError;
    
    fn write<W: Write>(mut w: W, a: &Self) -> Result<(), MapLoadError> {
        to_writer(&mut w, a)?;
        Ok(())
    }
    
    fn read<R: Read>(r: R) -> Result<Self, MapLoadError> {
        Ok(from_reader(r)?)
    }
}

#[derive(Debug)]
pub enum MapLoadError {
    Io(IoError),
    Serde(SerdeError),
}

impl Display for MapLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MapLoadError::Io(a) => write!(f, "{}", a),
            MapLoadError::Serde(a) => write!(f, "{}", a),
        }
    }
}

impl std::error::Error for MapLoadError {
    fn description(&self) -> &str {
        match self {
            MapLoadError::Io(e) => e.description(),
            MapLoadError::Serde(e) => e.description(),
        }
    }
}

impl From<IoError> for MapLoadError {
    fn from(a: IoError) -> MapLoadError {
        MapLoadError::Io(a)
    }
}

impl From<SerdeError> for MapLoadError {
    fn from(a: SerdeError) -> MapLoadError {
        MapLoadError::Serde(a)
    }
}

