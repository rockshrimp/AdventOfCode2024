
use std::collections::HashSet;
use advent_of_code_2024::{get_neighbours, read_lines, coordinates_in_bounds, Position};

fn is_valid_neighbour(width: i32, height: i32, pos: Position, plant_type: char, farm: &Vec<Vec<char>>) -> bool{
    return coordinates_in_bounds(width as i32, height as i32, pos) 
           && farm[pos.0 as usize][pos.1 as usize] == plant_type;
} 

fn find_region_perimeter(plot: Position, visited: &mut HashSet<Position>, farm: &Vec<Vec<char>>) -> usize{
    let height = farm.len();
    let width = farm[0].len();

    let mut queue = vec![plot];

    let mut perimeter = 0;
    let mut area = 0;
    let current_plant = farm[plot.0 as usize][plot.1 as usize];

    loop {
        match queue.pop(){
            Some(current_plot) =>{
                if visited.contains(&current_plot){
                    continue;
                }
                visited.insert(current_plot);

                let valid_neighbours: Vec<(i32, i32)> = get_neighbours(current_plot)
                    .into_iter()
                    .filter(|n| is_valid_neighbour(width as i32, height as i32, *n, current_plant, &farm))
                    .collect::<Vec<(i32, i32)>>();


                perimeter += 4 - valid_neighbours.len();
                area += 1;

                //println!("current_plant {:?} at pos {:?}", current_plant, current_plot);
                //println!("has {:?} neighbours", valid_neighbours.len());
                //println!("perimeter {:?}, area {:?}", perimeter, area);

                for neighbour in valid_neighbours{
                    if !visited.contains(&neighbour){
                        queue.push(neighbour);
                    }
                }
            },
            None => break,
        }
    }

    //println!("plant {:?}, price {:?}", current_plant, perimeter * area);

    return perimeter * area;

}

fn part_1(){
    let input = "input.txt";
    let farm: Vec<Vec<char>> = read_lines(&input).iter().map(|s| s.chars().collect::<Vec<char>>()).collect();

    let mut visited: HashSet<Position> = HashSet::new();
    let mut total_price = 0;
    for (x, line) in farm.iter().enumerate()
    {   
        for (y, _) in line.iter().enumerate(){
            let pos:Position = (x as i32, y as i32);
            if !visited.contains(&pos){
                total_price += find_region_perimeter(pos, &mut visited, &farm);
                println!("{:?}", total_price);

            }
            
        }
        //println!("{:?}", line);
    }
    println!("{:?}", total_price);

}

fn main(){
    part_1();
    //part_2();
}