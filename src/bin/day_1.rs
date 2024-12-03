use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn part_1(){
    let input = "input.txt";
    let lines = read_lines(&input);
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in lines.iter(){
        let mut line_split = line.split("   ");
        let left_val = line_split.next().unwrap().parse::<i32>().unwrap();
        let right_val = line_split.next().unwrap().parse::<i32>().unwrap();
        left.push(left_val);
        right.push(right_val);
    }

    
    left.sort();
    right.sort();
    let mut sum = 0;
    for i in 0..left.len(){
        sum += (right[i] - left[i]).abs()
    }

    println!("{}", sum);
}

fn main(){
    part_1();
}