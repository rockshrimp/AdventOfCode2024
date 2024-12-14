use advent_of_code_2024::{add_coordinates, coordinates_in_bounds, read_lines, substract_coordinates, Position};
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn get_norm(pos: Position) -> f64{
    return ((pos.0.pow(2) + pos.1.pow(2))as f64).sqrt();
}

fn get_antinode_positions(a:Position, b: Position) -> (Position, Position){
    if get_norm(a) > get_norm(b){
        let diff = substract_coordinates(a, b);
        //println!("{:?}, {:?}, {:?}", diff, add_coordinates(a, diff), substract_coordinates(b, diff));

        return (add_coordinates(a, diff), (substract_coordinates(b, diff)));
    }else{
        let diff = substract_coordinates(b, a);
        //println!("{:?}, {:?}, {:?}", diff, add_coordinates(a, diff), substract_coordinates(b, diff));

        return (add_coordinates(b, diff), (substract_coordinates(a, diff)));
    }
}

fn part_1(){
    let input = "input.txt";
    let lines: Vec<String> = read_lines(&input);

    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let mut char_map = HashMap::new();
    let mut antinodes = HashSet::new();

    for (x, lines) in lines.iter().enumerate(){
        for (y, c) in lines.chars().enumerate(){
            if c.is_alphanumeric(){
                char_map.entry(c)
                .or_insert_with(Vec::new) 
                .push((x as i32, y as i32));
            }
            
        }
    }

    for (c, pos_vec) in &char_map{
        let perms = pos_vec.iter().permutations(2).collect::<Vec<_>>();
        for perm in perms{
            let (antinode_a, antinode_b) = get_antinode_positions(*perm[0], *perm[1]);
            if coordinates_in_bounds(width, height, antinode_a){
                antinodes.insert(antinode_a);
            }
            if coordinates_in_bounds(width, height, antinode_b){
                antinodes.insert(antinode_b);
            } 
            //println!("{:?}", get_antinode_positions(*perm[0], *perm[1]));
        }
    }

    
    println!("{:?}", antinodes.len());
    
}   

fn main(){
    part_1();
    //part_2();
}