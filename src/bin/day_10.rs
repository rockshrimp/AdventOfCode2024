use std::collections::HashSet;

use advent_of_code_2024::{read_lines, Position, get_neighbours, coordinates_in_bounds};

fn get_trailhead_score(pos:Position, grid: &Vec<Vec<i32>>, width: usize, height: usize) -> i32{
    let mut queue: Vec<(i32, i32)> = vec![pos];
    let mut score = 0;
    let mut visited = HashSet::new();

    while queue.len() > 0{

        let pos = queue.pop().unwrap();
        visited.insert(pos);
        let val = grid[pos.0 as usize][pos.1 as usize];
        if val == 9{
            score += 1;
        }

        //println!("[{}, {}] val :{}", pos.0, pos.1, val);
        for neighbour in get_neighbours(pos){
            if coordinates_in_bounds(width as i32, height as i32, neighbour){
                if grid[neighbour.0 as usize][neighbour.1 as usize] == val + 1 && !visited.contains(&neighbour){
                    queue.push(neighbour);
                }
            }
        }
    }
    return score;
}

fn part_1(){
    let input = "input.txt";
    let lines: Vec<String> = read_lines(&input);
    
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for line in lines{
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect());
    }

    let height = grid.len();
    let width = grid[0].len();

    let mut scores_sum = 0;
    for (x, line) in grid.iter().enumerate(){
        //println!("{:?}", line);
        for (y, val) in line.iter().enumerate(){
            if *val == 0{
                //println!("0 found at {},{}", x, y);
                scores_sum += get_trailhead_score((x as i32, y as i32), &grid, width, height);
            }
        } 
    }

    println!("{}", scores_sum);


}

fn main(){
    part_1();
}