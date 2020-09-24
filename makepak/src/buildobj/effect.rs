use crate::input::EffectInput;
use anyhow::*;
use common::gamedata::{Effect, EffectKind};

macro_rules! need_field {
    ($e:expr, $field:ident) => {
        if let Some(field) = $e.$field {
            field
        } else {
            bail!("{} does not exist for effect kind")
        }
    };
}

pub fn convert_effect_input(e: Option<EffectInput>) -> Result<Option<Effect>> {
    let e = if let Some(e) = e {
        e
    } else {
        return Ok(None);
    };

    let mut kind = Vec::new();

    for k in &e.kind {
        kind.push(match k.kind.as_str() {
            "none" => EffectKind::None,
            "recover_hp" => EffectKind::RecoverHp,
            "recover_sp" => EffectKind::RecoverSp,
            "recover_mp" => EffectKind::RecoverMp,
            "melee" => EffectKind::Melee {
                element: need_field!(k, element),
            },
            "ranged" => EffectKind::Ranged {
                element: need_field!(k, element),
            },
            "explosion" => EffectKind::Explosion {
                element: need_field!(k, element),
            },
            "direct" => EffectKind::Direct {
                element: need_field!(k, element),
            },
            "status" => EffectKind::Status {
                status: need_field!(k, status),
            },
            "chara_scan" => EffectKind::CharaScan,
            _ => bail!("unknown field \"{}\" for effect kind"),
        });
    }

    Ok(Some(Effect {
        power_adjust: e.power_adjust,
        kind,
    }))
}
