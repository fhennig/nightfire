use crate::light::Coordinate;
use dualshock3::Coordinate as ControllerCoordinate;

pub fn controller_coordinate_to_coordinate(cc: &ControllerCoordinate) -> Coordinate {
    Coordinate(cc.0, cc.1)
}
