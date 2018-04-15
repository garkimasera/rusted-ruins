
use obj::*;
use pakutil::load_objs_dir;
use std::path::Path;

macro_rules! impl_idx {
    ($idx:ident, $obj:ty, $mem:ident) => {
        impl ObjectIndex for $idx {
            type ObjectType = $obj;
            fn get_obj_from_objholder<'a>(&self, objholder: &'a ObjectHolder) -> &'a $obj {
                &objholder.$mem[self.0 as usize]
            }

            fn to_id<'a>(&self, objholder: &'a ObjectHolder) -> &'a str {
                &objholder.$mem[self.0 as usize].id
            }

            fn search_idx(id: &str, objholder: &ObjectHolder) -> Option<$idx> {
                for (i, ref o) in (&objholder.$mem).into_iter().enumerate() {
                    if o.id == id {
                        return Some($idx(i as u32));
                    }
                }
                None
            }
        }

        impl FromId for $obj {
            fn get_obj_from_objholder_by_id<'a>(
                id: &str, objholder: &'a ObjectHolder) -> Option<&'a $obj> {

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
                &self.$mem[idx.0 as usize]
            }
        }

        impl Default for $idx {
            fn default() -> $idx {
                $idx(0)
            }
        }

        impl $idx {
            pub fn is_default(self) -> bool {
                self.0 == 0
            }
        }
            
    }
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
                    load_objs_dir(dir.as_ref(), |object| {
                        match object {
                            $(Object::$a(o) => { objholder.$mem.push(o); }),*
                        }
                    });
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
        }

        // Index type is an integer type that represents object index in ObjectHolder
        $(
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
            pub struct $idx(pub u32);

            impl_idx!($idx, $obj, $mem);
        )*
    }
}

impl_objholder! {
    {AnimImg, AnimImgObject, anim_img, AnimImgIdx},
    {CharaTemplate, CharaTemplateObject, chara_template, CharaTemplateIdx},
    {Deco, DecoObject, deco, DecoIdx},
    {Effect, EffectObject, effect, EffectIdx},
    {Item, ItemObject, item, ItemIdx},
    {SpecialTile, SpecialTileObject, special_tile, SpecialTileIdx},
    {Tile, TileObject, tile, TileIdx},
    {UIImg, UIImgObject, ui_img, UIImgIdx},
    {Wall, WallObject, wall, WallIdx},
    {MapTemplate, MapTemplateObject, map_template, MapTemplateIdx},
    {RegionGen, RegionGenObject, region_gen, RegionGenIdx},
    {SiteGen, SiteGenObject, site_gen, SiteGenIdx},
    {TalkScript, TalkScriptObject, talk_script, TalkScriptIdx}
}

pub trait ObjectIndex: Sized {
    type ObjectType;
    fn get_obj_from_objholder<'a>(&self, objholder: &'a ObjectHolder) -> &'a Self::ObjectType;
    fn to_id<'a>(&self, objholder: &'a ObjectHolder) -> &'a str;
    fn search_idx(id: &str, objholder: &ObjectHolder) -> Option<Self>;
}

pub trait FromId {
    fn get_obj_from_objholder_by_id<'a>(id: &str, objholder: &'a ObjectHolder) -> Option<&'a Self>;
}
    
pub trait Holder<I> {
    type ReturnType;
    fn get(&self, idx: I) -> &Self::ReturnType;
}

fn cmp_chara_template(a: &CharaTemplateObject, b: &CharaTemplateObject) -> ::std::cmp::Ordering {
    use std::cmp::Ordering;

    if a.id == "!" && b.id == "!" { return Ordering::Equal; }
    if a.id == "!" { return Ordering::Less; }
    if b.id == "!" { return Ordering::Greater; }
    let ord = a.race.cmp(&b.race);
    if ord != Ordering::Equal { return ord; }
    let ord = a.gen_level.cmp(&b.gen_level);
    if ord != Ordering::Equal { return ord; }
    a.id.cmp(&b.id)
}

