use crate::Vec2d;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShapeKind {
    OneTile,
    Line,
    Circle,
    // Sector,
    // All,
}

impl Default for ShapeKind {
    fn default() -> Self {
        ShapeKind::OneTile
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Shape {
    OneTile { pos: Vec2d },
    Line { start: Vec2d, end: Vec2d },
    Circle { center: Vec2d, radius: u32 },
}

impl Shape {
    pub fn is_inside(&self, p: Vec2d) -> bool {
        match *self {
            Shape::OneTile { pos } => pos == p,
            Shape::Line { .. } => unimplemented!(),
            Shape::Circle { center, radius } => (center.mdistance(p) as u32) < radius,
        }
    }
}
