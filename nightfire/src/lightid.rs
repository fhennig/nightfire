use crate::light;

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

    pub fn pos(&self) -> light::Coordinate {
        match &self {
            LightId::Top => light::Coordinate(-1.0, 1.0),
            LightId::Bottom => light::Coordinate(1.0, -1.0),
            LightId::Left => light::Coordinate(-1.0, -1.0),
            LightId::Right => light::Coordinate(1.0, 1.0),
        }
    }
}
