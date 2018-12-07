/// Helper crate to save maps for each file.

extern crate fnv;
extern crate serde;

mod ser;

use std::cell::Cell;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::path::{PathBuf, Path};
use std::io::{Write, Read, BufReader, BufWriter};
use std::fs::File;
use std::ops::{Deref, DerefMut};
use fnv::FnvHasher;

pub trait WithId: Sized {
    type ID;
    type Error: From<std::io::Error>;
    fn write<W: Write>(w: W, a: &Self) -> Result<(), Self::Error>;
    fn read<R: Read>(r: R) -> Result<Self, Self::Error>;
}

pub struct HashNamedFileBox<T: WithId> {
    id: T::ID,
    changed: Cell<bool>,
    inner: Option<Box<T>>,
}

impl<T: WithId> Deref for HashNamedFileBox<T> where T::ID: Debug {
    type Target = T;
    fn deref(&self) -> &T {
        if let Some(ref inner) = self.inner {
            inner
        } else {
            panic!("deref for unloaded object: {:?}", self.id);
        }
    }
}

impl<T: WithId> DerefMut for HashNamedFileBox<T> where T::ID: Debug {
    fn deref_mut(&mut self) -> &mut T {
        if let Some(ref mut inner) = self.inner {
            self.changed.set(true);
            inner
        } else {
            panic!("deref for unloaded object: {:?}", self.id);
        }
    }
}

impl<T: WithId> Debug for HashNamedFileBox<T> where T::ID: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.id)
    }
}

impl<T: WithId> HashNamedFileBox<T> where T::ID: Hash {
    pub fn new(id: T::ID, data: T) -> HashNamedFileBox<T> {
        HashNamedFileBox {
            id,
            changed: Cell::new(false),
            inner: Some(Box::new(data)),
        }
    }
    
    pub fn empty(id: T::ID) -> HashNamedFileBox<T> {
        HashNamedFileBox {
            id,
            changed: Cell::new(false),
            inner: None,
        }
    }

    pub fn id(&self) -> &T::ID {
        &self.id
    }

    pub fn write_force<P: AsRef<Path>>(s: &Self, p: P) -> Result<(), T::Error> {
        if let Some(a) = &s.inner {
            let mut file = BufWriter::new(File::create(s.path(p))?);
            T::write(&mut file, &a)?;
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
        p.as_ref().join(format!("{:x}", calc_hash(&self.id)))
    }

    pub fn read<P: AsRef<Path>>(&mut self, p: P) -> Result<(), T::Error> {
        if self.inner.is_some() {
            return Ok(());
        }
        
        let mut file = BufReader::new(File::open(self.path(p))?);
        self.inner = Some(Box::new(T::read(&mut file)?));
        
        Ok(())
    }
}

fn calc_hash<T: Hash>(a: &T) -> u64 {
    let mut hasher = FnvHasher::default();
    a.hash(&mut hasher);
    hasher.finish()
}
