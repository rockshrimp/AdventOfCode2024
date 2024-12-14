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

fn get_antinode_positions_2(a:Position, b: Position, width: i32, height: i32) -> HashSet<Position>{
    let mut antinodes = HashSet::new();
    //println!("{:?} & {:?}", a, b);

    if get_norm(a) > get_norm(b){
        let diff = substract_coordinates(a, b);
        let mut antinode_pos = a;

        while coordinates_in_bounds(width, height, antinode_pos){
            antinodes.insert(antinode_pos);
            //println!("{:?} + {:?}", antinode_pos, diff);
            antinode_pos = add_coordinates(antinode_pos, diff);
        }
        let mut antinode_pos = b;
        while coordinates_in_bounds(width, height, antinode_pos){
            antinodes.insert(antinode_pos);
            //println!("{:?} - {:?}", antinode_pos, diff);
            antinode_pos = substract_coordinates(antinode_pos, diff);
        }

        return antinodes;
        //return (add_coordinates(a, diff), (substract_coordinates(b, diff)));
    }else{
        let diff = substract_coordinates(b, a);
        //println!("{:?}, {:?}, {:?}", diff, add_coordinates(a, diff), substract_coordinates(b, diff));
        let mut antinode_pos = b;
        
        while coordinates_in_bounds(width, height, antinode_pos){
            antinodes.insert(antinode_pos);
            antinode_pos = add_coordinates(antinode_pos, diff);
            //println!("adding {:?} + {:?} = {:?}", antinode_pos, diff, add_coordinates(b, diff));
        }
        let mut antinode_pos: (i32, i32) = a;
        while coordinates_in_bounds(width, height, antinode_pos){
            antinodes.insert(antinode_pos);
            antinode_pos = substract_coordinates(antinode_pos, diff);
            //println!("{:?} - {:?}", antinode_pos, diff);
        }
        return antinodes;
    }
}

fn part_2(){
    let input = "input.txt";
    let lines: Vec<String> = read_lines(&input);

    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let mut char_map = HashMap::new();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (x, lines) in lines.iter().enumerate(){
        for (y, c) in lines.chars().enumerate(){
            if c.is_alphanumeric(){
                char_map.entry(c)
                .or_insert_with(Vec::new) 
                .push((x as i32, y as i32));
            }
        }
    }

    for (_, pos_vec) in &char_map{
        let perms = pos_vec.iter().permutations(2).collect::<Vec<_>>();
        for perm in perms{
            let new_antinodes = get_antinode_positions_2(*perm[0], *perm[1], width, height);
            antinodes.extend(new_antinodes);
            //println!("{:?}", get_antinode_positions(*perm[0], *perm[1]));
        }
    }
    //println!("{:?}", antinodes);

    println!("{:?}", antinodes.len());
    
}

fn main(){
    //part_1();
    part_2();
}