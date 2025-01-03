use std::{collections::HashMap, fs::read_to_string};
use advent_of_code_2024::Position;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn get_mod(val: i32, modulo: i32) -> i32{
    return ((val % modulo) + modulo) % modulo;
}

fn part_1(input: &str) -> isize{ 
    let re: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let width = 101;
    let height = 103;

    let mut quadrant = [0; 4];


    for (_, [p_x, p_y, v_x, v_y]) in re.captures_iter(input).map(|c| c.extract()) {
        let pos:Position = (p_x.parse().unwrap(), p_y.parse().unwrap());
        let velocity:Position = (v_x.parse().unwrap(), v_y.parse().unwrap());
        println!("{:?} {:?}", pos, velocity);

        let new_px = get_mod((pos.0 + velocity.0 * 100) % width, width);
        let new_py = get_mod((pos.1 + velocity.1 * 100) % height, height);

        //println!("{:?} {:?}, ({:?}, {:?})", pos, velocity, new_px, new_py);

        let left = new_px < width / 2;
        let right = new_px >= width / 2 + 1;
        let up = new_py < height / 2;
        let down = new_py >= height / 2 + 1;

        //println!("{:?} {:?} {:?} {:?}", left, up, right, down);

        if up && left{
            quadrant[0] += 1;
        }else if up && right{
            quadrant[1] += 1;
        }else if right && down{
            quadrant[2] += 1;
        }
        else if left && down{
            quadrant[3] += 1;
        }
        //println!("{:?}", quadrant);

        //println!("{:?}", quadrant.iter().fold(1, |acc, x| acc * x));
    }

    return quadrant.iter().fold(1, |acc, x| acc * x);
}


fn display_robots(robots: &Vec<(Robot)>){
    let width = 101;
    let height = 103;

    let mut grid = vec![vec!['.'; width]; height];
    for robot in robots{
        grid[robot.position_y as usize][robot.position_x as usize] = 'X';
    }
    for row in grid{
        println!("{}", row.into_iter().collect::<String>());
    }
}

struct Robot{
    position_x: i32,
    position_y: i32,
    velocity_x: i32,
    velocity_y: i32,
}



fn part_2(input: &str){ 
    let re: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let width: i32 = 101;
    let height: i32 = 103;

    let mut robots: Vec<Robot> = Vec::new();
    for (_, [p_x, p_y, v_x, v_y]) in re.captures_iter(input).map(|c| c.extract()) {
        let pos:Position = (p_x.parse().unwrap(), p_y.parse().unwrap());
        let velocity:Position = (v_x.parse().unwrap(), v_y.parse().unwrap());
        //println!("{:?} {:?}", pos, velocity);

        robots.push(Robot{
            position_x: pos.0,
            position_y: pos.1,
            velocity_x: velocity.0,
            velocity_y: velocity.1,
        });
    }

    let mut i = 1;
    let mut file: File = File::create("foo.txt").unwrap();
    
    loop {
        for robot in robots.iter_mut(){
            robot.position_x = get_mod((robot.position_x + robot.velocity_x) % width, width);
            robot.position_y = get_mod((robot.position_y + robot.velocity_y) % height, height);
        }

        // We store each robot y coordinates in a map 
        let mut robots_y = HashMap::new();
        for robot in &robots{
            *robots_y.entry(robot.position_y).or_insert(1) += 1;
        }

        // If we find more than 25 robots on the same line
        if *robots_y.values().max().unwrap() > 25{
            let mut grid = vec![vec!['.'; width as usize]; height as usize];
            for robot in &robots{
                grid[robot.position_y as usize][robot.position_x as usize] = 'X';
            }
            for row in grid{
                //println!("{}", row.into_iter().collect::<String>());
                let _ = file.write_all((row.into_iter().collect::<String>() + "\n").as_bytes());    
            }
            let _ = file.write_all((i.to_string() + "\n").as_bytes());    
        }
        i += 1;

        if i > 10000{
            break;
        }
    }
}


fn main(){
    let filename = "input.txt";
    let input = read_to_string(filename).unwrap();   

    //println!("{}", part_1(input.as_str()));
    part_2(input.as_str()); //7037
}

#[cfg(test)]

mod tests {
    use crate::part_1;

    const TEST: &str = 
"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

     #[test]
    fn test_part_1_1() {
        assert_eq!(part_1(TEST), 12);
    }
}