#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename = "snake_case")]
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
