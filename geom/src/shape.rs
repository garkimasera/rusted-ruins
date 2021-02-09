use crate::Vec2d;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Serialize, Deserialize)]
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

    pub fn iter(&self) -> Vec<Vec2d> {
        match *self {
            Shape::OneTile { pos } => vec![pos],
            Shape::Line { .. } => unimplemented!(),
            Shape::Circle { center, radius } => {
                if radius == 0 {
                    return vec![center];
                }
                let radius = radius as i32;
                let r = radius as f32 + 0.5;
                let r2 = r * r;
                super::RectIter::new(
                    center - Vec2d::new(radius, radius),
                    center + Vec2d::new(radius, radius),
                )
                .filter(|pos| pos.distance2(center) < r2)
                .collect()
            }
        }
    }
}
