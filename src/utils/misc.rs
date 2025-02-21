use crate::grid::position::GridPosition;

pub fn random_grid_position(width: i32, height: i32) -> GridPosition {
    let random_x = rand::random_range(0..width);
    let random_y = rand::random_range(0..height);

    GridPosition::new(width, height, random_x, random_y)
}
