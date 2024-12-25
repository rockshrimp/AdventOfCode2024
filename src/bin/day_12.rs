
use std::collections::HashSet;
use advent_of_code_2024::*;
use std::fs::read_to_string;

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

fn part_1(input: &str) -> usize{
    let farm: Vec<Vec<char>> = input
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>()).collect();

    let mut visited: HashSet<Position> = HashSet::new();
    let mut total_price = 0;
    for (x, line) in farm.iter().enumerate()
    {   
        for (y, _) in line.iter().enumerate(){
            let pos:Position = (x as i32, y as i32);
            if !visited.contains(&pos){
                total_price += find_region_perimeter(pos, &mut visited, &farm);
                //println!("{:?}", total_price);
            }
        }
    }

    println!("{:?}", total_price);
    return total_price;
}

fn find_angles(pos: Position, farm: &Vec<Vec<char>>, width: usize, height: usize) -> usize{
    let current_plant = farm[pos.0 as usize][pos.1 as usize];
    if let [U, D, L, R, U_R, U_L, D_R, D_L] = 
        get_neighbours_diagonal(pos)
            .iter()
            .map(|n| is_valid_neighbour(width as i32, height as i32, *n, current_plant, &farm))
            .collect::<Vec<bool>>().as_slice()
        {
                return vec![
                    *U && *L && ! *U_L, 
                    *U && *R && ! *U_R, 
                    *D && *L && ! *D_L, 
                    *D && *R && ! *D_R, 
                    !(*U || *L),
                    !(*U || *R),
                    !(*D || *L),
                    !(*D || *R)
                ].iter().map(|b| if *b == true{1} else{0}).sum();
        }else{
            return 0; 
        };
}

fn find_region_sides(plot: Position, visited: &mut HashSet<Position>, farm: &Vec<Vec<char>>) -> usize{
    let height = farm.len();
    let width = farm[0].len();

    let mut queue = vec![plot];
    let mut sides = 0;
    let mut area = 0;
    let current_plant = farm[plot.0 as usize][plot.1 as usize];

    while let Some(current_plot) = queue.pop(){
        if visited.contains(&current_plot){
            continue;
        }
        visited.insert(current_plot);
        area += 1;

        
        let angles_count = find_angles(current_plot, farm, width, height);
        sides += angles_count;

        let valid_neighbours: Vec<(i32, i32)> = get_neighbours(current_plot)
        .into_iter()
        .filter(|n| is_valid_neighbour(width as i32, height as i32, *n, current_plant, &farm))
        .collect::<Vec<(i32, i32)>>();
        //println!("current_plant {:?} has {:?} angles_count with area {:?}", current_plant, angles_count, area);

        for neighbour in valid_neighbours{
            if !visited.contains(&neighbour){
                queue.push(neighbour);
            }
        }
    }

    //println!("current_plant {:?} has {:?} sides with area {:?}", current_plant, sides, area);

    return sides * area;
}

fn part_2(input: &str) -> usize{
    let farm: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect::<Vec<char>>()).collect();

    let mut visited: HashSet<Position> = HashSet::new();
    let mut total_sides = 0;
    for (x, line) in farm.iter().enumerate()
    {   
        for (y, _) in line.iter().enumerate(){
            let pos:Position = (x as i32, y as i32);
            if !visited.contains(&pos){
                total_sides += find_region_sides(pos, &mut visited, &farm);
                //println!("{:?}", total_sides);
            }
        }
    }
    println!("{:?}", total_sides);
    return total_sides;
}

fn main(){
    let filename = "input.txt";
    let input = read_to_string(filename).unwrap();   

    part_1(input.as_str());
    part_2(input.as_str());
}

#[cfg(test)]
mod tests {
    use crate::part_1;
    use crate::part_2;

    const TEST: &str = 
"AAAA
BBCD
BBCC
EEEC";

const TEST_2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

     #[test]
    fn test_part_1_1() {
        assert_eq!(part_1(TEST), 140);
    }

    #[test]
    fn test_part_2_1() {
        assert_eq!(part_2(TEST), 80);
    }

    
    #[test]
    fn test_part_1_2() {
        assert_eq!(part_1(TEST_2), 772);
    }

    #[test]
    fn test_part_2_2() {
        assert_eq!(part_2(TEST_2), 436);
    }
}