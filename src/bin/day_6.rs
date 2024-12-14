use std::collections::{HashMap, HashSet};

use advent_of_code_2024::*;

fn find_start_position(lines: &Vec<Vec<char>>) -> (i32, i32){
    for (i, line) in lines.iter().enumerate(){
        if line.contains(&'^'){  
            let y_pos = line.iter().position(|c| *c == '^').unwrap();
            return (i as i32, y_pos as i32);
        }
    }
    return (0, 0);
}

fn part_1(){
    let input = "input.txt";
    let lines: Vec<Vec<char>> = read_lines(&input).iter().map(|s| s.chars().collect()).collect();

    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let mut current_position = find_start_position(&lines);

    let mut direction = UP;
    let mut visited:HashSet<Position> = HashSet::from([current_position]);

    let direction_change: HashMap<Position, Position> = HashMap::from(
        [(UP, RIGHT), (RIGHT, DOWN), (DOWN, LEFT), (LEFT, UP)]
    );

    loop{
        let new_pos = add_coordinates(current_position, direction);
        if coordinates_in_bounds(width, height, new_pos){
            if lines[new_pos.0 as usize][new_pos.1 as usize] == '#'{
                let new_direction = *direction_change.get(&direction).unwrap();
                //println!("Direction changed from {:?} {:?}", direction, new_direction);

                direction = new_direction;
            }
            else{
                visited.insert(new_pos);
                current_position = new_pos;
            }

            //println!("{:?}", current_position);
        }else{
            break;
        }
    }
    println!("{}", visited.len());
}

fn part_2(){
    let input = "input.txt";
    let mut lines: Vec<Vec<char>> = read_lines(&input).iter().map(|s| s.chars().collect()).collect();

    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let direction_change: HashMap<(i32, i32), (i32, i32)> = HashMap::from(
        [(UP, RIGHT), (RIGHT, DOWN), (DOWN, LEFT), (LEFT, UP)]
    );

    let mut obstructions_count = 0;
    
    for obstacle_x in 0..height{
        for obstacle_y in 0..width{
            if lines[obstacle_x as usize][obstacle_y as usize] == '#' || lines[obstacle_x as usize][obstacle_y as usize] == '^'{
                continue;
            }else{
                let mut line_with_obstacle = lines.clone();
                line_with_obstacle[obstacle_x as usize][obstacle_y as usize] = '#';

                let mut current_position = find_start_position(&lines);

                let mut direction = UP;
                let mut visited:HashSet<((i32, i32), (i32, i32))> = HashSet::from([(current_position, UP)]);

                loop{
                    let new_pos = add_coordinates(current_position, direction);

                    if coordinates_in_bounds(width, height, new_pos){
                        if line_with_obstacle[new_pos.0 as usize][new_pos.1 as usize] == '#'{
                            let new_direction = *direction_change.get(&direction).unwrap();
            
                            direction = new_direction;
                        }
                        else{
                            let already_visited = visited.insert((new_pos, direction));
                            if !already_visited{
                                //println!("found infinite loop");
                                obstructions_count += 1;
                                break;
                            }
                            current_position = new_pos;
                        }
                        //println!("{:?}", current_position);
                    }else{
                        break;
                    }
                }
            } 
        }
    }
    
    println!("{}", obstructions_count);
}

fn main(){
    //part_1();
    part_2();
}