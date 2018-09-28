
mod img;
mod item;

use array2d::Vec2d;
use common::obj::*;
use common::gamedata;
use tomlinput::TomlInput;
use error::*;
use self::img::*;
use self::item::build_item_object;

pub fn build_object(tomlinput: TomlInput) -> Result<Object, Error> {
    let object_type = tomlinput.object_type.clone();
    match object_type.as_ref() {
        "anim_img" => {
            return build_anim_img_object(tomlinput).map(|o| Object::AnimImg(o));
        }
        "chara_template" => {
            return build_chara_template_object(tomlinput).map(|o| Object::CharaTemplate(o));
        }
        "deco" => {
            return build_deco_object(tomlinput).map(|o| Object::Deco(o));
        }
        "effect" => {
            return build_effect_object(tomlinput).map(|o| Object::Effect(o));
        }
        "item" => {
            return build_item_object(tomlinput).map(|o| Object::Item(o));
        }
        "special_tile" => {
            return build_special_tile_object(tomlinput).map(|o| Object::SpecialTile(o));
        }
        "tile" => {
            return build_tile_object(tomlinput).map(|o| Object::Tile(o));
        }
        "ui_img" => {
            return build_ui_img_object(tomlinput).map(|o| Object::UIImg(o));
        }
        "wall" => {
            return build_wall_object(tomlinput).map(|o| Object::Wall(o));
        }
        "region_gen" => {
            return build_region_gen_object(tomlinput).map(|o| Object::RegionGen(o));
        }
        "script" => {
            return build_script_object(tomlinput).map(|o| Object::Script(o));
        }
        "site_gen" => {
            return build_site_gen_object(tomlinput).map(|o| Object::SiteGen(o));
        }
        "talk_script" => {
            return build_talk_script_object(tomlinput).map(|o| Object::TalkScript(o));
        }
        _ => {
            bail!("Unknown object_type");
        }
    }
}

fn build_deco_object(tomlinput: TomlInput) -> Result<DecoObject, Error> {
    let img = get_optional_field!(tomlinput, image);
    
    Ok(DecoObject {
        id: tomlinput.id,
        img: build_img(img)?.0,
    })
}

fn build_effect_object(tomlinput: TomlInput) -> Result<EffectObject, Error> {
    let img = get_optional_field!(tomlinput, image);
    
    Ok(EffectObject {
        id: tomlinput.id,
        img: build_img(img)?.0,
    })
}

fn build_special_tile_object(tomlinput: TomlInput) -> Result<SpecialTileObject, Error> {
    let img = get_optional_field!(tomlinput, image);
    let always_background = if let Some(special_tile) = tomlinput.special_tile {
        special_tile.always_background.unwrap_or(false)
    } else {
        false
    };
    
    Ok(SpecialTileObject {
        id: tomlinput.id,
        always_background: always_background,
        img: build_img(img)?.0,
    })
}

fn build_tile_object(tomlinput: TomlInput) -> Result<TileObject, Error> {
    let tile_dep_input = get_optional_field!(tomlinput, tile);
    let img = get_optional_field!(tomlinput, image);
    let (img, imgdata) = build_img(img)?;
    
    Ok(TileObject {
        id: tomlinput.id,
        img: img,
        kind: tile_dep_input.kind,
        symbol_color: imgdata.calc_average_color(),
    })
}

fn build_ui_img_object(tomlinput: TomlInput) -> Result<UIImgObject, Error> {
    let img = get_optional_field!(tomlinput, image);
    
    Ok(UIImgObject {
        id: tomlinput.id,
        img: build_img(img)?.0,
    })
}

fn build_wall_object(tomlinput: TomlInput) -> Result<WallObject, Error> {
    let img = get_optional_field!(tomlinput, image);
    let (img, imgdata) = build_img(img)?;
    let base_draw = if let Some(wall) = tomlinput.wall {
        wall.base_draw.unwrap_or(false)
    } else {
        true
    };
    
    Ok(WallObject {
        id: tomlinput.id,
        base_draw: base_draw,
        img: img,
        symbol_color: imgdata.calc_average_color(),
    })
}

fn build_chara_template_object(tomlinput: TomlInput) -> Result<CharaTemplateObject, Error> {    
    let chara_dep_input = get_optional_field!(tomlinput, chara_template);
    let img = get_optional_field!(tomlinput, image);
    
    Ok(CharaTemplateObject {
        id: tomlinput.id,
        img: build_img(img)?.0,
        race: chara_dep_input.race,
        gen_weight: chara_dep_input.gen_weight,
        gen_level: chara_dep_input.gen_level,
        default_ai_kind: chara_dep_input.default_ai_kind.unwrap_or(gamedata::NpcAIKind::None),
        max_hp: chara_dep_input.max_hp,
        str: chara_dep_input.str,
        vit: chara_dep_input.vit,
        dex: chara_dep_input.dex,
        int: chara_dep_input.int,
        wil: chara_dep_input.wil,
        cha: chara_dep_input.cha,
        spd: chara_dep_input.spd,
    })
}

fn build_anim_img_object(tomlinput: TomlInput) -> Result<AnimImgObject, Error> {
    let img = get_optional_field!(tomlinput, image);

    Ok(AnimImgObject {
        id: tomlinput.id,
        img: build_img(img)?.0,
    })
}

fn build_region_gen_object(tomlinput: TomlInput) -> Result<RegionGenObject, Error> {
    let rg = get_optional_field!(tomlinput, region_gen);
    use tomlinput::SiteGenIdAndPos;

    let f = |v: Vec<SiteGenIdAndPos>| -> Vec<(String, Vec2d)> {
        v.into_iter().map(|a| (a.id, a.pos)).collect()
    };
    

    Ok(RegionGenObject {
        id: tomlinput.id,
        map_template_id: rg.map_template_id,
        towns: f(rg.towns),
    })
}

fn build_script_object(tomlinput: TomlInput) -> Result<ScriptObject, Error> {
    use common::script;
    let s = get_optional_field!(tomlinput, script);
    let script = match script::parse(&s.script) {
        Ok(o) => o,
        Err(e) => bail!(PakCompileError::ScriptParseError { description: e.to_string() } ),
    };

    Ok(ScriptObject {
        id: tomlinput.id,
        script,
    })
}

fn build_site_gen_object(tomlinput: TomlInput) -> Result<SiteGenObject, Error> {
    let sg = get_optional_field!(tomlinput, site_gen);

    Ok(SiteGenObject {
        id: tomlinput.id,
        kind: sg.kind,
        map_template_id: sg.map_template_id,
        unique_citizens: sg.unique_citizens.unwrap_or(vec![]),
        shops: sg.shops.unwrap_or(vec![]),
    })
}

fn build_talk_script_object(tomlinput: TomlInput) -> Result<TalkScriptObject, Error> {
    let talk_script_dep = get_optional_field!(tomlinput, talk_script);
    use common::hashmap::HashMap;
    use common::talkscript::*;
    let mut sections: HashMap<String, TalkSection> = HashMap::default();
    for (k, v) in talk_script_dep.sections {

        let section = match v.kind {
            TalkSectionKind::Normal => {
                if v.answers.len() != v.dest_sections.len() {
                    bail!("Answer_texts and dest_sections have different length");
                }
                TalkSection::Normal {
                    text: v.text,
                    answers: v.answers,
                    dest_sections: v.dest_sections,
                    default_dest_section: v.default_dest_section,
                }
            }
            TalkSectionKind::Reaction => {
                if let Some(trigger) = v.trigger {
                    let reaction = TalkReaction::EventTrigger {
                        trigger: trigger,
                    };
                    TalkSection::Reaction {
                        reaction,
                        next_section: get_optional_field!(v, next_section),
                    }
                } else {
                    bail!("field for reaction needed");
                }
            }
            TalkSectionKind::Special => {
                TalkSection::Special {
                    special: get_optional_field!(v, special),
                    dest_sections: v.dest_sections,
                }
                    
            }
        };
        
        sections.insert(k, section);
    }

    Ok(TalkScriptObject {
        id: tomlinput.id,
        sections: sections,
    })
}

