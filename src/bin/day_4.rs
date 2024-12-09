use advent_of_code_2024::read_lines;

fn east(x:usize, y:usize, grid: &Vec<String>, word: &str)-> bool{
    return grid[x][y..y + word.len()] == *word;
}

fn west(x:usize, y:usize, grid: &Vec<String>, word: &str)-> bool{
    //println!("{} {} {} found west match", x, y, &grid[x][y - word.len()..y]);
    return grid[x][y + 1 - word.len()..y + 1] == *word.chars().rev().collect::<String>()
}

fn south(x:usize, y:usize, grid: &Vec<String>, word: &str)-> bool{
    let mut column = String::new();
    for i in x..x+word.len(){
        column.push(grid[i].chars().nth(y).unwrap());
    } 
    
    if column == word{
        //println!("{} {} {} found south match", x, y, column);
    }
    return column == word;
}

fn north(x:usize, y:usize, grid: &Vec<String>, word: &str)-> bool{
    let mut column = String::new();
    for i in 0..word.len(){
        column.push(grid[x - i].chars().nth(y).unwrap());
    }

    if column == word{
        //println!("{} {} {} found north match", x, y, column);
    }
    return column == word;
}

fn north_east(x:usize, y:usize, grid: &Vec<String>, word: &str)-> bool{
    let mut diag = String::new(); 
    for i in 0..word.len(){
        diag.push(grid[x - i].chars().nth(y + i).unwrap());
    }

    if diag == word{
        //println!("{} {} {} found north-east match", x, y, diag);
    }

    return diag == word;
}

fn south_east(x:usize, y:usize, grid: &Vec<String>, word: &str)-> bool{
    let mut diag = String::new(); 
    for i in 0..word.len(){
        diag.push(grid[x + i].chars().nth(y + i).unwrap());
    }
    
    if diag == word{
        //println!("{} {} {} found south-east match", x, y, diag);
    }
    
    return diag == word;
}

fn north_west(x:usize, y:usize, grid: &Vec<String>, word: &str)-> bool{
    let mut diag = String::new(); 
    for i in 0..word.len(){
        diag.push(grid[x - i].chars().nth(y - i).unwrap());
    }
    
    if diag == word{
        //println!("{} {} {} found north-west match", x, y, diag);
    }
    return diag == word;
}

fn south_west(x:usize, y:usize, grid: &Vec<String>, word: &str)-> bool{
    let mut diag = String::new(); 
    for i in 0..word.len(){
        diag.push(grid[x + i].chars().nth(y - i).unwrap());
    }
    
    if diag == word{
        //println!("{} {} {} found south-west match", x, y, diag);
    }
    return diag == word;
}



fn detect_word(x:usize, y:usize, grid: &Vec<String>) -> i32 {
    let word = "XMAS";
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    let valid_east = y + word.len() <= width as usize;
    let valid_west = y as i32 + 1 - word.len() as i32 >= 0;
    let valid_north = x as i32 >= word.len() as i32 - 1 as i32;
    let valid_south = x as i32 <= height - word.len() as i32;
    
    let mut count = 0;

    if valid_east{
        if east(x, y, grid, word){
            count += 1;
        }
    }

    if valid_west{
        if west(x, y, grid, word){
            //println!("{} {} {} found west match", x, y, &grid[x][y - word.len()..y]);
            count += 1;
        }
    }

    if valid_south{
        if south(x, y, grid, word){
            //println!("{} {} {} found west match", x, y, &grid[x][y - word.len()..y]);
            count += 1;
        }
    }

    if valid_north{
        if north(x, y, grid, word){
            //println!("{} {} {} found west match", x, y, &grid[x][y - word.len()..y]);
            count += 1;
        }
    }

    if valid_north && valid_east{
        if north_east(x, y, grid, word){
            //println!("{} {} {} found west match", x, y, &grid[x][y - word.len()..y]);
            count += 1;
        }
    }
    
    if valid_south && valid_east{
        if south_east(x, y, grid, word){
            //println!("{} {} {} found west match", x, y, &grid[x][y - word.len()..y]);
            count += 1;
        }
    }

    if valid_north && valid_west{
        if north_west(x, y, grid, word){
            //println!("{} {} {} found west match", x, y, &grid[x][y - word.len()..y]);
            count += 1;
        }
    }

    if valid_south && valid_west{
        if south_west(x, y, grid, word){
            //println!("{} {} {} found west match", x, y, &grid[x][y - word.len()..y]);
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
        for (y, ch) in lines[x].chars().enumerate(){
            if ch == 'X'{
                result += detect_word(x, y, &lines);
            }
        }
    }
    println!("result : {}", result);
}

fn detect_word_2(x:usize, y:usize, grid: &Vec<String>) -> i32 {
    let word = "MAS";
    let width = grid[0].len();
    let height = grid.len();
    let mut count = 0;

    if x > 0 && x + 1 < height && y > 0 && y + 1 < width{
        //if east{
        if east(x, y, grid, word){
            count += 1;
        }

        
        //if west{
        if grid[x][y - 1 ..y + 2] == *word.chars().rev().collect::<String>(){
            println!("{} {} {} found west match", x, y, &grid[x][y - word.len()..y]);
            count += 1;
        }
        
        /*
        let mut column = String::new();
        for i in x..x+word.len(){
            column.push(grid[i].chars().nth(y).unwrap());
        } 
        if column == word{
            //println!("{} {} {} found south match", x, y, column);

            count += 1;
        }
    
    
        let mut column = String::new();
        for i in 0..word.len(){
            column.push(grid[x - i].chars().nth(y).unwrap());
        }

        if column == word{
            //println!("{} {} {} found north match", x, y, column);
            count += 1;
        }
    
    
        let mut diag = String::new(); 
        for i in 0..word.len(){
            diag.push(grid[x - i].chars().nth(y + i).unwrap());
        }

        if diag == word{
            //println!("{} {} {} found north-east match", x, y, diag);
            count += 1;
        }
        
        let mut diag = String::new(); 
        for i in 0..word.len(){
            diag.push(grid[x + i].chars().nth(y + i).unwrap());
        }

        if diag == word{
            //println!("{} {} {} found south-east match", x, y, diag);
            count += 1;
        }
    
        let mut diag = String::new(); 
        for i in 0..word.len(){
            diag.push(grid[x - i].chars().nth(y - i).unwrap());
        }

        if diag == word{
            //println!("{} {} {} found north-west match", x, y, diag);
            count += 1;
        }

        let mut diag = String::new(); 
        for i in 0..word.len(){
            diag.push(grid[x + i].chars().nth(y - i).unwrap());
        }

        if diag == word{
            //println!("{} {} {} found south-west match", x, y, diag);
            count += 1;
        }
        */
    }

    return 0;
}

fn part_2(){
    let input = "input.txt";
    let lines = read_lines(&input);

    let mut result = 0;
    for x in 0..lines.len(){
        for (y, ch) in lines[x].chars().enumerate(){
            if ch == 'A'{
                println!("Found A at {} {}", x, y);
                result += detect_word_2(x, y, &lines);
            }
        }
    }
    println!("result : {}", result);
}

fn main(){
    part_1();
    //part_2();
}