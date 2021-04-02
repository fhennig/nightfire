use dualshock3::Coordinate as ControllerCoordinate;
use nightfire::light::Coordinate;

pub fn controller_coordinate_to_coordinate(cc: &ControllerCoordinate) -> Coordinate {
    Coordinate(cc.0, cc.1)
}
