
//! Random Map Generator for Rusted Ruins

extern crate rand;
extern crate rusted_ruins_array2d as array2d;

use array2d::*;

mod lattice;
mod fractal;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileKind {
    Floor, Wall, Door,
}

pub struct GeneratedMap {
    pub size: Vec2d,
    pub tile: Array2d<TileKind>,
}

#[derive(Clone, PartialEq, Debug)]
enum MapGenParam {
    Flat,
    Lattice { nx: u32, ny: u32, step_min: u32, step_max: u32, door_weight: f64 },
}

pub struct MapGenerator {
    map: GeneratedMap,
    genparam: Option<MapGenParam>,
}

impl MapGenerator {
    pub fn new<S: Into<Vec2d>>(size: S) -> MapGenerator {
        let size = size.into();
        let map = GeneratedMap {
            size, tile: Array2d::new(size.0 as u32, size.1 as u32, TileKind::Floor),
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
        self, nx: u32, ny: u32, step_min: u32, step_max: u32, door_weight: f64) -> MapGenerator {
        
        let mut mg = self;
        mg.genparam = Some(MapGenParam::Lattice { nx, ny, step_min, step_max, door_weight } );
        mg
    }

    /// Generate one map
    pub fn generate(mut self) -> GeneratedMap {
        match self.genparam.expect("Map generate before giving parameters") {
            MapGenParam::Flat => {
                return self.map;
            },
            MapGenParam::Lattice { nx, ny, step_min, step_max, door_weight } => {
                let lattice = lattice::create_lattice(nx, ny, step_min, step_max);
                lattice.write_to_map(&mut self.map, door_weight);
                return self.map;
            },
        }
    }
}

impl std::fmt::Display for GeneratedMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for ny in 0..self.size.1 {
            for nx in 0..self.size.0 {
                let c = match self.tile[(nx, ny)] {
                    TileKind::Floor => '.',
                    TileKind::Wall  => '#',
                    TileKind::Door  => 'D',
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

        println!("Flat map:");
        println!("{}", map);
    }

    #[test]
    fn lattice_map() {
        let map = MapGenerator::new((19, 15)).lattice(5, 4, 3, 7, 0.5).generate();

        println!("Lattice map:");
        println!("{}", map);
    }

    #[test]
    fn fractal_map() {
        let fractal = fractal::create_fractal(Vec2d::new(30, 30));

        println!("Fractal map:");
        let mut map = MapGenerator::new((30, 30)).flat().generate();
        fractal::write_to_map(&mut map);
        println!("{}", map);
    }
}
