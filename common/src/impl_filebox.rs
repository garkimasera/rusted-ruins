use crate::gamedata::Map;
use filebox::*;
use serde_cbor::{error::Error as SerdeError, from_reader, ser::to_writer_packed};
use std::io::{Error as IoError, Read, Write};
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum MapLoadError {
    #[error("io error")]
    Io(#[from] IoError),
    #[error("serde error")]
    Serde(#[from] SerdeError),
}
