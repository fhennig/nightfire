use crate::light::coord;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum LightId {
    Top,
    Bottom,
    Left,
    Right,
}

impl LightId {
    pub fn all() -> Vec<LightId> {
        vec![LightId::Top, LightId::Bottom, LightId::Left, LightId::Right]
    }

    pub fn pos(&self) -> coord::Coordinate {
        match &self {
            LightId::Top => coord::Coordinate(-1.0, 1.0),
            LightId::Bottom => coord::Coordinate(1.0, -1.0),
            LightId::Left => coord::Coordinate(-1.0, -1.0),
            LightId::Right => coord::Coordinate(1.0, 1.0),
        }
    }
}
