use std::fs::read_to_string;

pub type Position = (i32, i32);

pub const UP: Position = (-1, 0);
pub const DOWN: Position = (1, 0);
pub const LEFT: Position = (0, -1);
pub const RIGHT: Position = (0, 1);

pub fn add_coordinates(pos_a: Position, pos_b:Position) -> Position{
    return (pos_a.0 + pos_b.0, pos_a.1 + pos_b.1);
}

pub fn substract_coordinates(pos_a: Position, pos_b:Position) -> Position{
    return (pos_a.0 - pos_b.0, pos_a.1 - pos_b.1);
}

pub fn get_neighbours(pos: Position) -> Vec<Position>{
    let mut neighbours = Vec::new();
    for dir in vec![UP, DOWN, LEFT, RIGHT]{
        neighbours.push(add_coordinates(pos, dir));
    }
    return neighbours;
}

pub fn coordinates_in_bounds(width: i32, height: i32, pos: Position) -> bool{
    let (x, y)= pos;
    if x < 0 || x >= height{
        return false;
    }
    if y < 0 || y >= width{
        return false;
    }
    return true;
}

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}