use crate::gamedata::Map;
use filebox::*;
use serde_cbor::{error::Error as SerdeError, from_reader, ser::to_writer_packed};
use std::fmt::Display;
use std::io::{Error as IoError, Read, Write};

impl WithId for Map {
    type Error = MapLoadError;

    fn write<W: Write>(mut w: W, a: &Self) -> Result<(), MapLoadError> {
        to_writer_packed(&mut w, a)?;
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
