
use array2d::*;
use common::obj::CharaTemplateObject;
use common::objholder::*;
use common::chara::*;
use common::item::Inventory;
use super::Game;
use obj;

pub fn add_chara(game: &mut Game, chara: Chara, pos: Option<Vec2d>, ty: CharaType) {
    let chara_place = game.chara_holder.add(chara, ty);
    if let Some(pos) = pos {
        game.current_map.add_character(pos, chara_place);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum CharaType {
    /// Default value.
    Unknown,
    /// Player is unique character in the game.
    Player,
    /// Indexed for a map. This character don't appear on other maps.
    OnMap,
}

const N_CHARA_TYPE: usize = 2;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum CharaId {
    Unknown,
    Player,
    OnMap(u32),
}

impl Default for CharaId {
    fn default() -> CharaId {
        CharaId::Unknown
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct CharaHolder {
    pub player: Chara,
    pub on_map: Vec<Chara>,
}

impl CharaHolder {
    pub fn get<'a>(&'a self, id: CharaId) -> &'a Chara {
        match id {
            CharaId::Unknown => panic!("Invalid charaid"),
            CharaId::Player => {
                &self.player
            },
            CharaId::OnMap(i) => {
                &self.on_map[i as usize]
            },
        }
    }

    pub fn get_mut<'a>(&'a mut self, id: CharaId) -> &'a mut Chara {
        match id {
            CharaId::Unknown => panic!("Invalid charaid"),
            CharaId::Player => {
                &mut self.player
            },
            CharaId::OnMap(i) => {
                &mut self.on_map[i as usize]
            },
        }
    }

    pub fn add(&mut self, chara: Chara, ty: CharaType) -> CharaId {
        match ty {
            CharaType::Unknown => panic!("Invalid charatype"),
            CharaType::Player => {
                self.player = chara;
                CharaId::Player
            },
            CharaType::OnMap => {
                self.on_map.push(chara);
                CharaId::OnMap(self.on_map.len() as u32 - 1)
            }
        }
    }

    pub fn id_iter_on_map(&self) -> CharaIdIter {
        CharaIdIter {
            i: 0, j: 0, v: [Some((CharaType::OnMap, self.on_map.len() as u32)), None]
        }
    }

    pub fn relative_relation(&self, a: CharaId, b: CharaId) -> Relationship {
        let a = self.get(a);
        let b = self.get(b);
        a.rel.relative(b.rel)
    }
}

pub fn create_chara(idx: CharaTemplateIdx, level: u32) -> Chara {
    //let template = obj::get_obj(idx);
    let id = obj::idx_to_id(idx);
    
    Chara {
        name: ::text::obj_txt(id).to_owned(),
        params: CharaParams::default(),
        template: obj::idx_to_id(idx).to_owned(),
        template_idx: idx,
        inventory: Inventory::for_chara(),
        wait_time: 100.0,
        rel: Relationship::NEUTRAL,
    }
}

/// This iterator returns CharaId over CharaHolder.
#[derive(Clone, Copy)]
pub struct CharaIdIter {
    i: usize,
    j: u32,
    v: [Option<(CharaType, u32)>; N_CHARA_TYPE], // Array of targeted CharaType and its max value
}

impl Iterator for CharaIdIter {
    type Item = CharaId;
    fn next(&mut self) -> Option<CharaId> {
        if self.i == N_CHARA_TYPE || self.v[self.i] == None {
            return None;
        }
        
        let return_value;
        let v = self.v[self.i].unwrap();
        match v.0 {
            CharaType::Unknown => panic!("Invalid charaid"),
            CharaType::Player => {
                return_value = Some(CharaId::Player);
                self.i += 1;
            },
            CharaType::OnMap => {
                return_value = Some(CharaId::OnMap(self.j));
                if self.j == v.1 - 1 {
                    self.i += 1;
                    self.j = 0;
                }else{
                    self.j += 1;
                }
            },
        }
        return_value
    }
}
