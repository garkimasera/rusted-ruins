//! Random Map Generator for Rusted Ruins

#![warn(
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style
)]

extern crate rusted_ruins_geom as geom;
extern crate rusted_ruins_rng as rng;

use arrayvec::ArrayVec;
use geom::*;
use serde_derive::{Deserialize, Serialize};

pub mod binary;

mod fractal;
mod lattice;
mod rooms;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileKind {
    Floor,
    Wall,
    Door,
}

impl TileKind {
    pub fn is_passable(self) -> bool {
        match self {
            TileKind::Floor | TileKind::Door => true,
            TileKind::Wall => false,
        }
    }
}

#[derive(Clone)]
pub enum Entrance {
    /// Tile position
    Pos(ArrayVec<Vec2d, 4>),
    /// the later is for deeper floor
    Stairs(Vec2d, Option<Vec2d>),
}

pub struct GeneratedMap {
    pub size: Vec2d,
    pub tile: Array2d<TileKind>,
    pub entrance: Entrance,
    pub exit: Option<Vec2d>,
}

impl GeneratedMap {
    pub fn new<S: Into<Vec2d>>(size: S) -> GeneratedMap {
        let size = size.into();
        let mut v = ArrayVec::new();
        v.push(Vec2d(0, 0));
        GeneratedMap {
            size,
            tile: Array2d::new(size.0 as u32, size.1 as u32, TileKind::Floor),
            entrance: Entrance::Pos(v),
            exit: None,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum MapGenParam {
    Flat {
        w: u32,
        h: u32,
    },
    Lattice {
        w: u32,
        h: u32,
        nx: u32,
        ny: u32,
        step_min: u32,
        step_max: u32,
        door_weight: f32,
    },
    Fractal {
        w: u32,
        h: u32,
        stairs: bool,
        edge: bool,
        wall_weight: f32,
    },
    Rooms {
        w: u32,
        h: u32,
        max_room_size: u32,
        min_room_size: u32,
        n_room: u32,
    },
}

impl MapGenParam {
    pub fn size(&self) -> (u32, u32) {
        match *self {
            MapGenParam::Flat { w, h } => (w, h),
            MapGenParam::Lattice { w, h, .. } => (w, h),
            MapGenParam::Fractal { w, h, .. } => (w, h),
            MapGenParam::Rooms { w, h, .. } => (w, h),
        }
    }

    pub fn generate(&self) -> GeneratedMap {
        match *self {
            MapGenParam::Flat { w, h } => GeneratedMap::new((w, h)),
            MapGenParam::Lattice {
                w,
                h,
                nx,
                ny,
                step_min,
                step_max,
                door_weight,
            } => {
                let mut map = GeneratedMap::new((w, h));
                let lattice = lattice::create_lattice(nx, ny, step_min, step_max);
                lattice.write_to_map(&mut map, door_weight);
                map
            }
            MapGenParam::Fractal {
                w,
                h,
                stairs,
                edge,
                wall_weight,
            } => {
                let mut map = GeneratedMap::new((w, h));
                fractal::write_to_map(&mut map, wall_weight, edge, stairs);
                map
            }
            MapGenParam::Rooms {
                w,
                h,
                max_room_size,
                min_room_size,
                n_room,
            } => {
                let mut map = GeneratedMap::new((w, h));
                let rooms = rooms::Rooms::new(max_room_size, min_room_size, n_room);
                rooms.write_to_map(&mut map);
                map
            }
        }
    }
}

impl Default for MapGenParam {
    fn default() -> Self {
        MapGenParam::Flat { w: 1, h: 1 }
    }
}

impl Entrance {
    fn entrance_char(&self, pos: Vec2d) -> Option<char> {
        match *self {
            Entrance::Pos(ref v) => {
                if v.iter().any(|p| pos == *p) {
                    Some('e')
                } else {
                    None
                }
            }
            Entrance::Stairs(e0, e1) => {
                if e0 == pos {
                    Some('<')
                } else if e1 == Some(pos) {
                    Some('>')
                } else {
                    None
                }
            }
        }
    }
}

impl std::fmt::Display for GeneratedMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ny in 0..self.size.1 {
            for nx in 0..self.size.0 {
                let c = if let Some(c) = self.entrance.entrance_char(Vec2d(nx, ny)) {
                    c
                } else {
                    match self.tile[(nx, ny)] {
                        TileKind::Floor => '.',
                        TileKind::Wall => '#',
                        TileKind::Door => 'D',
                    }
                };

                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn flat_map() {
        let gen_param = MapGenParam::Flat { w: 10, h: 10 };
        let map = gen_param.generate();

        println!("Flat map:\n{}", map);
    }

    #[test]
    fn lattice_map() {
        let gen_param = MapGenParam::Lattice {
            w: 19,
            h: 15,
            nx: 5,
            ny: 4,
            step_min: 3,
            step_max: 7,
            door_weight: 0.5,
        };
        let map = gen_param.generate();

        println!("Lattice map:\n{}", map);
    }

    #[test]
    fn fractal_map() {
        let gen_param = MapGenParam::Fractal {
            w: 30,
            h: 30,
            stairs: true,
            edge: true,
            wall_weight: 0.5,
        };
        let map = gen_param.generate();
        println!("Fractal map:\n{}", map);
    }

    #[test]
    fn rooms_map() {
        let gen_param = MapGenParam::Rooms {
            w: 35,
            h: 35,
            min_room_size: 5,
            max_room_size: 8,
            n_room: 7,
        };
        let map = gen_param.generate();
        println!("Rooms map:\n{}", map);
    }
}
