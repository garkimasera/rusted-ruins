//! Random Map Generator for Rusted Ruins

extern crate rusted_ruins_array2d as array2d;
extern crate rusted_ruins_rng as rng;

use array2d::*;

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

pub struct GeneratedMap {
    pub size: Vec2d,
    pub tile: Array2d<TileKind>,
    pub entrance: Vec2d,
    pub exit: Option<Vec2d>,
}

#[derive(Clone, PartialEq, Debug)]
enum MapGenParam {
    Flat,
    Lattice {
        nx: u32,
        ny: u32,
        step_min: u32,
        step_max: u32,
        door_weight: f64,
    },
    Fractal,
    Rooms {
        max_room_size: u32,
        min_room_size: u32,
        n_room: u32,
    },
}

pub struct MapGenerator {
    map: GeneratedMap,
    genparam: Option<MapGenParam>,
}

impl MapGenerator {
    pub fn new<S: Into<Vec2d>>(size: S) -> MapGenerator {
        let size = size.into();
        let map = GeneratedMap {
            size,
            tile: Array2d::new(size.0 as u32, size.1 as u32, TileKind::Floor),
            entrance: Vec2d(0, 0),
            exit: None,
        };
        MapGenerator {
            map,
            genparam: None,
        }
    }

    /// Create flat map. Flat map is consisted of floor tile only.
    pub fn flat(self) -> MapGenerator {
        let mut mg = self;
        mg.genparam = Some(MapGenParam::Flat);
        mg
    }

    /// Create lattice map. There are separated rooms in lattice
    pub fn lattice(
        self,
        nx: u32,
        ny: u32,
        step_min: u32,
        step_max: u32,
        door_weight: f64,
    ) -> MapGenerator {
        let mut mg = self;
        mg.genparam = Some(MapGenParam::Lattice {
            nx,
            ny,
            step_min,
            step_max,
            door_weight,
        });
        mg
    }

    /// Create fractal map
    pub fn fractal(self) -> MapGenerator {
        let mut mg = self;
        mg.genparam = Some(MapGenParam::Fractal);
        mg
    }

    pub fn rooms(mut self, min_room_size: u32, max_room_size: u32, n_room: u32) -> MapGenerator {
        self.genparam = Some(MapGenParam::Rooms {
            max_room_size,
            min_room_size,
            n_room,
        });
        self
    }

    /// Generate one map
    pub fn generate(mut self) -> GeneratedMap {
        match self
            .genparam
            .expect("Map generate before giving parameters")
        {
            MapGenParam::Flat => {
                return self.map;
            }
            MapGenParam::Lattice {
                nx,
                ny,
                step_min,
                step_max,
                door_weight,
            } => {
                let lattice = lattice::create_lattice(nx, ny, step_min, step_max);
                lattice.write_to_map(&mut self.map, door_weight);
                return self.map;
            }
            MapGenParam::Fractal => {
                fractal::write_to_map(&mut self.map);
                return self.map;
            }
            MapGenParam::Rooms {
                max_room_size,
                min_room_size,
                n_room,
            } => {
                let rooms = rooms::Rooms::new(max_room_size, min_room_size, n_room);
                rooms.write_to_map(&mut self.map);
                return self.map;
            }
        }
    }
}

impl std::fmt::Display for GeneratedMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for ny in 0..self.size.1 {
            for nx in 0..self.size.0 {
                let c = if self.entrance == (nx, ny) {
                    '<'
                } else if self.exit == Some(Vec2d(nx, ny)) {
                    '>'
                } else {
                    match self.tile[(nx, ny)] {
                        TileKind::Floor => '.',
                        TileKind::Wall => '#',
                        TileKind::Door => 'D',
                    }
                };

                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn flat_map() {
        let map = MapGenerator::new((10, 10)).flat().generate();

        println!("Flat map:\n{}", map);
    }

    #[test]
    fn lattice_map() {
        let map = MapGenerator::new((19, 15))
            .lattice(5, 4, 3, 7, 0.5)
            .generate();

        println!("Lattice map:\n{}", map);
    }

    #[test]
    fn fractal_map() {
        let map = MapGenerator::new((30, 30)).fractal().generate();
        println!("Fractal map:\n{}", map);
    }

    #[test]
    fn rooms_map() {
        let map = MapGenerator::new((35, 35)).rooms(5, 8, 7).generate();
        println!("Rooms map:\n{}", map);
    }
}
