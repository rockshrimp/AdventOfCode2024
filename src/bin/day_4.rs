use advent_of_code_2024::read_lines;

fn detect_word(x:usize, y:usize, grid: &Vec<String>) -> i32 {
    let word = "XMAS";
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    let east = y + word.len() <= width as usize;
    let west = y as i32 + 1 - word.len() as i32 >= 0;
    let north = x as i32 >= word.len() as i32 - 1 as i32;
    let south = x as i32 <= height - word.len() as i32;
    
    let mut count = 0;

    if east{
        if grid[x][y..y + word.len()] == *word{
            //println!("{} {} found east match", x, y);
            count += 1;
        }
    }

    if west{
        if grid[x][y + 1 - word.len()..y + 1] == *word.chars().rev().collect::<String>(){
            //println!("{} {} {} found west match", x, y, &grid[x][y - word.len()..y]);
            count += 1;
        }
    }

    if south{
        let mut column = String::new();
        for i in x..x+word.len(){
            column.push(grid[i].chars().nth(y).unwrap());
        } 
        if column == word{
            //println!("{} {} {} found south match", x, y, column);

            count += 1;
        }
    }

    if north{
        let mut column = String::new();
        for i in 0..word.len(){
            column.push(grid[x - i].chars().nth(y).unwrap());
        }

        if column == word{
            //println!("{} {} {} found north match", x, y, column);
            count += 1;
        }
    }

    if north && east{
        let mut diag = String::new(); 
        for i in 0..word.len(){
            diag.push(grid[x - i].chars().nth(y + i).unwrap());
        }

        if diag == word{
            //println!("{} {} {} found north-east match", x, y, diag);
            count += 1;
        }
    }
    
    //south-east
    if south && east {
        let mut diag = String::new(); 
        for i in 0..word.len(){
            diag.push(grid[x + i].chars().nth(y + i).unwrap());
        }

        if diag == word{
            //println!("{} {} {} found south-east match", x, y, diag);
            count += 1;
        }
    }

    if north && west{
        let mut diag = String::new(); 
        for i in 0..word.len(){
            diag.push(grid[x - i].chars().nth(y - i).unwrap());
        }

        if diag == word{
            //println!("{} {} {} found north-west match", x, y, diag);
            count += 1;
        }
    }

    if south && west{
        let mut diag = String::new(); 
        for i in 0..word.len(){
            diag.push(grid[x + i].chars().nth(y - i).unwrap());
        }

        if diag == word{
            //println!("{} {} {} found south-west match", x, y, diag);
            count += 1;
        }
    }

    return count;
}

fn part_1(){
    let input = "input.txt";
    let lines = read_lines(&input);

    let mut result = 0;
    for x in 0..lines.len(){
        for y in 0..lines[0].len(){
            //println!("{} {}", x, y);
            result += detect_word(x, y, &lines);
        }
    }
    println!("result : {}", result);
}

fn main(){
    part_1();
    //part_2();
}