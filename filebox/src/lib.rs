/// Helper crate to save maps for each file.
mod ser;

use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::cell::Cell;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

pub trait WithId: Sized {
    type Error: From<std::io::Error>;
    fn write<W: Write>(w: W, a: &Self) -> Result<(), Self::Error>;
    fn read<R: Read>(r: R) -> Result<Self, Self::Error>;
}

pub struct FileBox<T> {
    id: u64,
    changed: Cell<bool>,
    inner: Option<Box<T>>,
}

impl<T: WithId> Deref for FileBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        if let Some(ref inner) = self.inner {
            inner
        } else {
            panic!("deref for unloaded object: {:016x}", self.id);
        }
    }
}

impl<T: WithId> DerefMut for FileBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        if let Some(ref mut inner) = self.inner {
            self.changed.set(true);
            inner
        } else {
            panic!("deref for unloaded object: {:016x}", self.id);
        }
    }
}

impl<T: WithId> Debug for FileBox<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.id)
    }
}

impl<T: WithId> FileBox<T> {
    pub fn new(id: u64, data: T) -> FileBox<T> {
        FileBox {
            id,
            changed: Cell::new(true),
            inner: Some(Box::new(data)),
        }
    }

    pub fn empty(id: u64) -> FileBox<T> {
        FileBox {
            id,
            changed: Cell::new(false),
            inner: None,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn write_force<P: AsRef<Path>>(s: &Self, p: P) -> Result<(), T::Error> {
        if let Some(a) = &s.inner {
            let mut file = GzEncoder::new(
                BufWriter::new(File::create(s.path(p))?),
                Compression::fast(),
            );
            T::write(&mut file, a)?;
            s.changed.set(false);
        }

        Ok(())
    }

    pub fn write<P: AsRef<Path>>(s: &Self, p: P) -> Result<(), T::Error> {
        if s.changed.get() {
            Self::write_force(s, p)
        } else {
            Ok(())
        }
    }

    pub fn path<P: AsRef<Path>>(&self, p: P) -> PathBuf {
        p.as_ref().join(format!("{:016x}", self.id))
    }

    pub fn read<P: AsRef<Path>>(&mut self, p: P) -> Result<(), T::Error> {
        if self.inner.is_some() {
            return Ok(());
        }

        let mut file = GzDecoder::new(BufReader::new(File::open(self.path(p))?));
        self.inner = Some(Box::new(T::read(&mut file)?));

        Ok(())
    }
}
