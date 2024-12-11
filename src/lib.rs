use std::fs::read_to_string;

pub const UP: (i32, i32) = (-1, 0);
pub const DOWN: (i32, i32) = (1, 0);
pub const LEFT: (i32, i32) = (0, -1);
pub const RIGHT: (i32, i32) = (0, 1);

pub fn add_coordinates(pos_a: (i32, i32), pos_b:(i32, i32)) -> (i32, i32){
    return (pos_a.0 + pos_b.0, pos_a.1 + pos_b.1);
}

pub fn coordinates_in_bounds(width: i32, height: i32, pos: (i32, i32)) -> bool{
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