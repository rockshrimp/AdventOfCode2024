use std::ops::Sub;

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
            loop{
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
    return get_score(grid);
}

#[derive(Debug, Clone, Copy)]
struct Cell{
    pos: Position,
    char: char
}

fn part_2(input: &str) -> usize{ 
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut directions: Vec<char> = Vec::new();
    let mut current_pos: Position = (0, 0); 

    let lines: Vec<String> = read_lines(input);

    parse_input(&mut grid, &mut directions, lines, &mut current_pos);
    let mut doubled_grid: Vec<Vec<char>> = Vec::new();
    for (x, row) in grid.iter().enumerate(){
        let mut new_row: Vec<char> = Vec::new();
        for (y, c) in row.iter().enumerate(){
            if *c == 'O'{
                new_row.append(&mut vec!['[', ']']);
            }else if *c == '@'{
                new_row.append(&mut vec!['@', '.']);
                current_pos = (x as i32, y as i32 *2);
            }else{
                new_row.append(&mut vec![*c, *c]);
            }
        }
        doubled_grid.push(new_row);
    }

    for row in &doubled_grid{
        println!("{:?}", row.iter().collect::<String>());
    }


    let box_chars = vec!['[', ']'];

    for (i, char) in directions.iter().enumerate(){
        let dir: (i32, i32) = char_to_dir(*char);
        let mut new_pos = add_coordinates(current_pos, dir);
        println!("{:?}, {:?}, {:?}", new_pos, char, dir);

        if doubled_grid[new_pos.0 as usize][new_pos.1 as usize] == '.'{
            doubled_grid[new_pos.0 as usize][new_pos.1 as usize] = '@';
            doubled_grid[current_pos.0 as usize][current_pos.1 as usize] = '.';
            current_pos = new_pos;   
            println!("no boxes found, moving {:?} to {:?}",doubled_grid[current_pos.0 as usize][current_pos.1 as usize], new_pos);  
        }else if box_chars.contains(&doubled_grid[new_pos.0 as usize][new_pos.1 as usize]){
            if dir == LEFT || dir == RIGHT{
                let current_cell = Cell{pos: current_pos, char:doubled_grid[current_pos.0 as usize][current_pos.1 as usize]};
                let new_cell = Cell{pos: new_pos, char:doubled_grid[new_pos.0 as usize][new_pos.1 as usize]};
                let mut box_cells = vec![current_cell, new_cell];
                loop{
                    new_pos = add_coordinates(new_pos, dir);
                    if box_chars.contains(&doubled_grid[new_pos.0 as usize][new_pos.1 as usize]){
                        //println!("adding box");
                        box_cells.push(Cell{pos: new_pos, char:doubled_grid[new_pos.0 as usize][new_pos.1 as usize]});
                    }else if doubled_grid[new_pos.0 as usize][new_pos.1 as usize] == '#'{
                         break;
                    }else if doubled_grid[new_pos.0 as usize][new_pos.1 as usize] == '.'{
                        //println!("{:?}", box_cells);
                        let mut prev_pos = new_pos;
                        for cell in box_cells.iter().rev(){
                            doubled_grid[prev_pos.0 as usize][prev_pos.1 as usize] = cell.char;
                            prev_pos = cell.pos;
                        }
                        doubled_grid[prev_pos.0 as usize][prev_pos.1 as usize] = '.';
                        current_pos = add_coordinates(current_pos, dir);
                        break;
                    } 
                }
            }else{
                let current_cell = Cell{pos: current_pos, char:doubled_grid[current_pos.0 as usize][current_pos.1 as usize]};
                let mut box_rows: Vec<Vec<Cell>> = vec![vec![current_cell]];
                let mut row_idx = 0;
                'outer: loop{
                    let mut new_row: Vec<Cell> = vec![];
                    for (idx, cell) in box_rows[row_idx].iter().enumerate(){
                        println!("checking cell {:?}", cell);

                        new_pos = add_coordinates(cell.pos, dir);
                        if doubled_grid[new_pos.0 as usize][new_pos.1 as usize] == '#'{
                            break 'outer;
                        }else{
                            if doubled_grid[new_pos.0 as usize][new_pos.1 as usize] == ']' && idx == 0{
                                new_row.push(Cell{pos: add_coordinates(new_pos, LEFT), char: '[' });
                            }
                            new_row.push(Cell{pos: new_pos, char: doubled_grid[new_pos.0 as usize][new_pos.1 as usize]});
                            if doubled_grid[new_pos.0 as usize][new_pos.1 as usize] == '[' && idx == box_rows[row_idx].len() - 1{
                                new_row.push(Cell{pos: add_coordinates(new_pos, RIGHT), char: ']' });
                            }
                        }
                    }

                    println!("{:?}", new_row);


                    if new_row.iter().all(|&c| c.char == '.'){
                        println!("moving boxes vertically");
                        for row in box_rows.iter().rev(){
                            for cell in row{
                                let moved_pos = add_coordinates(cell.pos, dir);
                                doubled_grid[moved_pos.0 as usize][moved_pos.1 as usize] = cell.char;
                                doubled_grid[cell.pos.0 as usize][cell.pos.1 as usize] = '.';
                            }
                        }
                        current_pos = add_coordinates(current_pos, dir);
                        break 'outer;
                    }
                    box_rows.push(new_row);
                    row_idx += 1;
                }
            }
        }

        let mut flag = false;
        for row in &doubled_grid{
            for (idx, c) in row.iter().enumerate(){
                if *c == ']' && row[idx - 1] != '['{
                    flag = true;
                }
            }
            println!("{:?}", row.iter().collect::<String>());
        }
        if flag || i > 50{
            break;
        }

    } 

    return 0;
}
    
fn main(){
    let filename = "input.txt";

    //println!("{}", part_1(filename));
    println!("{}", part_2(filename));
}

