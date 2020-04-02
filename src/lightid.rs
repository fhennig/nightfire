use crate::models::{Coordinate, Positionable};

#[derive(juniper::GraphQLEnum, Debug, PartialEq, Eq, Hash, Copy, Clone)]
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
}

impl Positionable for LightId {
    fn pos(&self) -> Coordinate {
        match &self {
            LightId::Top => Coordinate(-1.0, 1.0),
            LightId::Bottom => Coordinate(1.0, -1.0),
            LightId::Left => Coordinate(-1.0, -1.0),
            LightId::Right => Coordinate(1.0, 1.0),
        }
    }
}
