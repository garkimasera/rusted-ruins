mod img;
mod item;

use self::img::*;
use self::item::build_item_object;
use crate::input::Input;
use anyhow::*;
use common::gamedata::CharaBaseAttr;
use common::obj::*;
use geom::Vec2d;

pub fn build_object(input: Input) -> Result<Object, Error> {
    let object_type = input.object_type.clone();
    match object_type.as_ref() {
        "anim_img" => build_anim_img_object(input).map(Object::AnimImg),
        "chara_template" => build_chara_template_object(input).map(Object::CharaTemplate),
        "deco" => build_deco_object(input).map(Object::Deco),
        "effect_img" => build_effect_object(input).map(Object::EffectImg),
        "item" => build_item_object(input).map(Object::Item),
        "special_tile" => build_special_tile_object(input).map(Object::SpecialTile),
        "tile" => build_tile_object(input).map(Object::Tile),
        "ui_img" => build_ui_img_object(input).map(Object::UiImg),
        "wall" => build_wall_object(input).map(Object::Wall),
        "region_gen" => build_region_gen_object(input).map(Object::RegionGen),
        "script" => build_script_object(input).map(Object::Script),
        "site_gen" => build_site_gen_object(input).map(Object::SiteGen),
        _ => {
            bail!("Unknown object_type");
        }
    }
}

fn build_deco_object(input: Input) -> Result<DecoObject, Error> {
    let img = get_optional_field!(input, image);

    Ok(DecoObject {
        id: input.id,
        img: build_img(img)?.0,
    })
}

fn build_effect_object(input: Input) -> Result<EffectImgObject, Error> {
    let img = get_optional_field!(input, image);

    Ok(EffectImgObject {
        id: input.id,
        img: build_img(img)?.0,
    })
}

fn build_special_tile_object(input: Input) -> Result<SpecialTileObject, Error> {
    let img = get_optional_field!(input, image);
    let always_background = if let Some(special_tile) = input.special_tile {
        special_tile.always_background.unwrap_or(false)
    } else {
        false
    };

    Ok(SpecialTileObject {
        id: input.id,
        always_background,
        img: build_img(img)?.0,
    })
}

fn build_tile_object(input: Input) -> Result<TileObject, Error> {
    let tile_dep_input = get_optional_field!(input, tile);
    let img = get_optional_field!(input, image);
    let (img, imgdata) = build_img(img)?;

    Ok(TileObject {
        id: input.id,
        img,
        kind: tile_dep_input.kind,
        symbol_color: imgdata.calc_average_color(),
    })
}

fn build_ui_img_object(input: Input) -> Result<UiImgObject, Error> {
    let img = get_optional_field!(input, image);

    Ok(UiImgObject {
        id: input.id,
        img: build_img(img)?.0,
    })
}

fn build_wall_object(input: Input) -> Result<WallObject, Error> {
    let img = get_optional_field!(input, image);
    let (img, imgdata) = build_img(img)?;
    let (hp, base_draw, build_skill, materials, mining_rewards) = if let Some(wall) = input.wall {
        (
            wall.hp.unwrap_or(0xFFFF),
            wall.base_draw.unwrap_or(false),
            wall.build_skill,
            wall.materials,
            wall.mining_rewards,
        )
    } else {
        (0xFFFF, true, None, None, vec![])
    };

    Ok(WallObject {
        id: input.id,
        hp,
        base_draw,
        img,
        symbol_color: imgdata.calc_average_color(),
        build_skill,
        materials,
        mining_rewards,
    })
}

fn build_chara_template_object(input: Input) -> Result<CharaTemplateObject, Error> {
    let chara_dep_input = get_optional_field!(input, chara_template);
    let img = get_optional_field!(input, image);

    let base_attr = CharaBaseAttr {
        base_hp: chara_dep_input.base_hp,
        str: chara_dep_input.str as i16,
        vit: chara_dep_input.vit as i16,
        dex: chara_dep_input.dex as i16,
        int: chara_dep_input.int as i16,
        wil: chara_dep_input.wil as i16,
        cha: chara_dep_input.cha as i16,
        spd: chara_dep_input.spd as i16,
    };

    Ok(CharaTemplateObject {
        id: input.id,
        img: build_img(img)?.0,
        race: chara_dep_input.race,
        gen_weight: chara_dep_input.gen_weight,
        gen_level: chara_dep_input.gen_level,
        default_ai_kind: chara_dep_input.default_ai_kind,
        skill_bonus: chara_dep_input.skill_bonus,
        base_attr,
    })
}

fn build_anim_img_object(input: Input) -> Result<AnimImgObject, Error> {
    let img = get_optional_field!(input, image);

    Ok(AnimImgObject {
        id: input.id,
        img: build_img(img)?.0,
    })
}

fn build_region_gen_object(input: Input) -> Result<RegionGenObject, Error> {
    let rg = get_optional_field!(input, region_gen);
    use crate::input::SiteGenIdAndPos;

    let f = |v: Vec<SiteGenIdAndPos>| -> Vec<(String, Vec2d)> {
        v.into_iter().map(|a| (a.id, a.pos)).collect()
    };

    Ok(RegionGenObject {
        id: input.id,
        map_template_id: rg.map_template_id,
        towns: f(rg.towns),
        others: f(rg.others),
    })
}

fn build_script_object(input: Input) -> Result<ScriptObject, Error> {
    let s = get_optional_field!(input, script);

    Ok(ScriptObject {
        id: input.id,
        script: s.script,
    })
}

fn build_site_gen_object(input: Input) -> Result<SiteGenObject, Error> {
    let sg = get_optional_field!(input, site_gen);

    Ok(SiteGenObject {
        id: input.id,
        kind: sg.kind,
        site_symbol: sg.site_symbol,
        default_faction_id: sg.default_faction_id,
        map_template_id: sg.map_template_id,
        unique_citizens: sg.unique_citizens.unwrap_or_default(),
        shops: sg.shops.unwrap_or_default(),
    })
}
