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
            LightId::Top => Coordinate(-0.5, 0.5),
            LightId::Bottom => Coordinate(0.5, -0.5),
            LightId::Left => Coordinate(-0.5, -0.5),
            LightId::Right => Coordinate(0.5, 0.5),
        }
    }
}
