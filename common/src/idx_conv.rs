pub use crate::objholder::IdxConvTable;
use once_cell::sync::Lazy;
use std::sync::RwLock;

pub(crate) static IDX_CONV_TABLE: Lazy<RwLock<Option<IdxConvTable>>> =
    Lazy::new(|| RwLock::new(None));

pub fn set_idx_conv_table(idx_conv_table: Option<IdxConvTable>) {
    *IDX_CONV_TABLE.write().expect("IDX_CONV_TABLE lock error") = idx_conv_table;
}

#[macro_export]
macro_rules! idx_conv {
    ($({$a:ident, $obj:ty, $mem:ident, $idx:ident}),*) => {
        /// Table to convert idx from savefiles that have another id table
        #[derive(Default)]
        pub struct IdxConvTable {
            $(
                $mem: Vec<u32>,
            )*
        }

        impl IdxConvTable {
            #[cfg(feature="global_state_obj")]
            pub fn read<R: std::io::BufRead>(mut r: R, dest_hash: u64)
                                             -> Result<Option<IdxConvTable>, anyhow::Error> {
                use $crate::basic::ID_TABLE_SECTION_TAG;

                let hash = {
                    let mut buf = String::new();
                    r.read_line(&mut buf)?;
                    u64::from_str_radix(buf.trim(), 16)?
                };
                if hash == dest_hash {
                    return Ok(None)
                }

                let mut table = IdxConvTable::default();
                let mut current_obj_type = String::new();

                for line in r.lines() {
                    let line = line?;
                    if line.starts_with(ID_TABLE_SECTION_TAG) {
                        current_obj_type =
                            line.trim_start_matches(ID_TABLE_SECTION_TAG).trim_end().to_owned();
                    } else {
                        match current_obj_type.as_str() {
                            $(
                                stringify!($obj) => {
                                    if let Some(dest_idx) = $crate::gobj::id_to_idx_checked::<$idx>(&line) {
                                        table.$mem.push(dest_idx.as_usize() as u32);
                                    } else {
                                        table.$mem.push($idx::default().as_usize() as u32);
                                    }
                                }
                            )*
                            _ => {
                                warn!("unknown section name \"{}\" in idtable", &current_obj_type);
                            }
                        }
                    }
                }

                Ok(Some(table))
            }

            $(
                pub fn $mem(&self, i: $idx) -> $idx {
                    if let Some(i) = self.$mem.get(i.as_usize()).copied() {
                        $idx::from_usize(i as usize)
                    } else {
                        warn!("invalid value in index conversion: {}({})", stringify!($idx), i.as_usize());
                        $idx::from_usize(0)
                    }
                }
            )*
        }

        $(
            #[cfg(feature="global_state_obj")]
            impl<'de> serde::Deserialize<'de> for $idx {
                fn deserialize<D>(deserializer: D) -> Result<$idx, D::Error>
                where D: serde::Deserializer<'de> {

                    let lock = $crate::idx_conv::IDX_CONV_TABLE.read().expect("IDX_CONV_TABLE lock error");
                    let i = u32::deserialize(deserializer)?;
                    let idx = $idx::from_raw_int(i).unwrap(); // TODO: Should generate error if None
                    if let Some(idx_conv_table) = lock.as_ref() {
                        Ok(idx_conv_table.$mem(idx))
                    } else {
                        Ok(idx)
                    }
                }
            }

            #[cfg(not(feature="global_state_obj"))]
            impl<'de> serde::Deserialize<'de> for $idx {
                fn deserialize<D>(deserializer: D) -> Result<$idx, D::Error>
                where D: serde::Deserializer<'de> {
                    panic!("Deserialize index type without global_state_obj")
                }
            }
        )*
    }
}
