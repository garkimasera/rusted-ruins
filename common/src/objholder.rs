use crate::obj::*;
use crate::pakutil::load_objs_dir;
use std::num::NonZeroU32;
use std::path::Path;

const NON_ZERO_U32_1: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(1) };

macro_rules! impl_idx {
    ($idx:ident, $obj:ty, $mem:ident) => {
        impl ObjectIndex for $idx {
            type ObjectType = $obj;
            fn get_obj_from_objholder<'a>(&self, objholder: &'a ObjectHolder) -> &'a $obj {
                &objholder.$mem[self.as_usize()]
            }

            fn to_id<'a>(&self, objholder: &'a ObjectHolder) -> &'a str {
                &objholder.$mem[self.as_usize()].id
            }

            fn search_idx(id: &str, objholder: &ObjectHolder) -> Option<$idx> {
                for (i, ref o) in (&objholder.$mem).into_iter().enumerate() {
                    if o.id == id {
                        return Some($idx::from_usize(i));
                    }
                }
                None
            }

            fn as_raw_int(&self) -> u32 {
                self.0.get()
            }

            fn from_raw_int(i: u32) -> Option<Self> {
                Some($idx(NonZeroU32::new(i)?))
            }
        }

        impl FromId for $obj {
            fn get_obj_from_objholder_by_id<'a>(
                id: &str,
                objholder: &'a ObjectHolder,
            ) -> Option<&'a $obj> {
                for ref o in (&objholder.$mem).into_iter() {
                    if o.id == id {
                        return Some(o);
                    }
                }
                None
            }
        }

        impl Holder<$idx> for ObjectHolder {
            type ReturnType = $obj;
            fn get<'a>(&'a self, idx: $idx) -> &'a $obj {
                &self.$mem[idx.as_usize()]
            }
        }

        impl Default for $idx {
            #[inline(always)]
            fn default() -> $idx {
                $idx(NON_ZERO_U32_1)
            }
        }

        impl $idx {
            /// Return inner value as usize
            #[inline(always)]
            pub fn as_usize(self) -> usize {
                self.0.get() as usize - 1
            }

            /// Creae from usize
            #[inline(always)]
            pub fn from_usize(i: usize) -> Self {
                $idx(unsafe { NonZeroU32::new_unchecked((i + 1) as u32) })
            }

            #[inline(always)]
            pub fn is_default(self) -> bool {
                self.0 == NON_ZERO_U32_1
            }
        }
    };
}

macro_rules! impl_objholder {
    ($({$a:ident, $obj:ty, $mem:ident, $idx:ident}),*) => {
        pub struct ObjectHolder {
            $(pub $mem: Vec<$obj>),*
        }

        impl ObjectHolder {
            pub fn new() -> ObjectHolder {
                ObjectHolder {
                    $($mem: Vec::new()),*
                }
            }

            pub fn load<P: AsRef<Path>>(dirs: &[P]) -> ObjectHolder {
                let mut objholder = ObjectHolder::new();

                for dir in dirs {
                    let err_stack = load_objs_dir(dir.as_ref(), |object| {
                        match object {
                            $(Object::$a(o) => { objholder.$mem.push(o); }),*
                        }
                    });

                    if !err_stack.is_empty() {
                        warn!("object loading error in {}\n{:?}", dir.as_ref().to_string_lossy(), err_stack);
                    }
                }

                objholder.sort();
                objholder
            }

            fn sort(&mut self) {
                {
                    $(self.$mem.sort_by(|a, b| a.id.cmp(&b.id)));*
                }

                // chara_template is sorted by the special function
                // because the order is used for choosing chara from race and gen_level
                self.chara_template.sort_by(|a, b| cmp_chara_template(a, b));
            }

            /// Write id table
            pub fn write_table<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
                $({
                    write!(w, "{}{}\n", crate::basic::ID_TABLE_SECTION_TAG, stringify!($obj))?;
                    for o in &self.$mem {
                        write!(w, "{}\n", &o.id)?;
                    }
                })*
                Ok(())
            }

            pub fn debug_print(&self, ty: &str) {
                $({
                    if ty == stringify!($mem) {
                        for o in &self.$mem {
                            println!("{}", o.id);
                        }
                        return;
                    }
                })*
            }
        }

        /// Hash is used to verify the identity of ObjectHolder and id table.
        impl std::hash::Hash for ObjectHolder {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                $({
                    for o in &self.$mem {
                        o.id.hash(state);
                    }
                    crate::basic::ID_TABLE_SECTION_TAG.hash(state);
                })*
            }
        }

        idx_conv!(
            $({$a, $obj, $mem, $idx}),*
        );

        // Index type is an integer type that represents object index in ObjectHolder
        $(
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
            #[serde(transparent)]
            pub struct $idx(NonZeroU32);

            impl_idx!($idx, $obj, $mem);
        )*
    }
}

impl_objholder! {
    {AnimImg, AnimImgObject, anim_img, AnimImgIdx},
    {CharaTemplate, CharaTemplateObject, chara_template, CharaTemplateIdx},
    {Deco, DecoObject, deco, DecoIdx},
    {EffectImg, EffectImgObject, effect_img, EffectImgIdx},
    {Item, ItemObject, item, ItemIdx},
    {SpecialTile, SpecialTileObject, special_tile, SpecialTileIdx},
    {Tile, TileObject, tile, TileIdx},
    {UIImg, UIImgObject, ui_img, UIImgIdx},
    {Wall, WallObject, wall, WallIdx},
    {MapTemplate, MapTemplateObject, map_template, MapTemplateIdx},
    {RegionGen, RegionGenObject, region_gen, RegionGenIdx},
    {Script, ScriptObject, script, ScriptIdx},
    {SiteGen, SiteGenObject, site_gen, SiteGenIdx}
}

pub trait ObjectIndex: Sized {
    type ObjectType;
    fn get_obj_from_objholder<'a>(&self, objholder: &'a ObjectHolder) -> &'a Self::ObjectType;
    fn to_id<'a>(&self, objholder: &'a ObjectHolder) -> &'a str;
    fn search_idx(id: &str, objholder: &ObjectHolder) -> Option<Self>;
    fn as_raw_int(&self) -> u32;
    fn from_raw_int(i: u32) -> Option<Self>;
}

pub trait FromId {
    fn get_obj_from_objholder_by_id<'a>(id: &str, objholder: &'a ObjectHolder) -> Option<&'a Self>;
}

pub trait Holder<I> {
    type ReturnType;
    fn get(&self, idx: I) -> &Self::ReturnType;
}

fn cmp_chara_template(a: &CharaTemplateObject, b: &CharaTemplateObject) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    if a.id == "!" && b.id == "!" {
        return Ordering::Equal;
    }
    if a.id == "!" {
        return Ordering::Less;
    }
    if b.id == "!" {
        return Ordering::Greater;
    }
    let ord = a.race.cmp(&b.race);
    if ord != Ordering::Equal {
        return ord;
    }
    let ord = a.gen_level.cmp(&b.gen_level);
    if ord != Ordering::Equal {
        return ord;
    }
    a.id.cmp(&b.id)
}
