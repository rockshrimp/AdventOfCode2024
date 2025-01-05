use advent_of_code_2024::{add_coordinates, read_lines, Position, DOWN, LEFT, RIGHT, UP};

fn parse_input(grid: &mut Vec<Vec<char>>, directions: &mut Vec<char>, lines:  Vec<String>, start_position: &mut Position){
    for (idx, line) in lines.iter().enumerate(){
        if line.starts_with("#"){
            grid.push(line.chars().collect());

            match line.find('@') {
                Some(y) => *start_position = (idx as i32, y as i32),
                _ => ()
            }
        }else if line.starts_with(&['v', '^', '>', '<']) {
            for c in line.chars(){
                directions.push(c);
            }
        }
    }
}

fn char_to_dir(c:char) -> Position{
    match c {
        '<' => LEFT,
        '^' => UP,
        '>' => RIGHT,
        'v' => DOWN,
        _ => (0,0),
    }
}

fn get_score(grid: Vec<Vec<char>>) -> usize{
    let mut score = 0;
    for (x, row) in grid.iter().enumerate(){
        for (y, c) in row.iter().enumerate(){
            if grid[x][y] == 'O'{
                score += 100 * x + y;
            }
        }
    }
    return score;
}

fn part_1(input: &str) -> usize{ 
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut directions: Vec<char> = Vec::new();
    let mut current_pos: Position = (0, 0); 


    let lines: Vec<String> = read_lines(input);

    parse_input(&mut grid, &mut directions, lines, &mut current_pos);
    /*for row in &grid{
        println!("{:?}", row.iter().collect::<String>());
    }*/
    //println!("{:?} {:?}", current_pos, directions);

    for (i, char) in directions.iter().enumerate(){
        let dir = char_to_dir(*char);

        let mut new_pos = add_coordinates(current_pos, dir);
        if grid[new_pos.0 as usize][new_pos.1 as usize] == '.'{
            grid[new_pos.0 as usize][new_pos.1 as usize] = '@';
            grid[current_pos.0 as usize][current_pos.1 as usize] = '.';
            current_pos = new_pos;     
        }else if grid[new_pos.0 as usize][new_pos.1 as usize] == 'O'{
            let mut box_positions = vec![new_pos];
            loop {
                new_pos = add_coordinates(new_pos, dir);
                if grid[new_pos.0 as usize][new_pos.1 as usize] == 'O'{
                    box_positions.push(new_pos);
                }else if grid[new_pos.0 as usize][new_pos.1 as usize] == '#'{
                     break;
                }else if grid[new_pos.0 as usize][new_pos.1 as usize] == '.'{
                    grid[box_positions[0].0 as usize][box_positions[0].1 as usize] = '@';
                    grid[new_pos.0 as usize][new_pos.1 as usize] = 'O';
                    grid[current_pos.0 as usize][current_pos.1 as usize] = '.';
                    current_pos = box_positions[0];     
                    break;
                }
            }
        }

        /*for row in &grid{
            println!("{:?}", row.iter().collect::<String>());
        }*/
    } 
    return get_score(grid);;
}

fn part_2(input: &str){ 
}
    
fn main(){
    let filename = "input.txt";

    println!("{}", part_1(filename));
    //part_2(input.as_str()); //7037
}

