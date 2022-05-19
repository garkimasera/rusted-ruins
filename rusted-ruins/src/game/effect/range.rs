use common::gamedata::*;
use geom::*;

/// Get range of the given effect
pub fn effect_to_range(effect: &Effect, center: Coords) -> Shape {
    Shape::Circle {
        center,
        radius: effect.range,
    }
}
