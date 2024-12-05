#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn is_safe(values: &Vec<i32>) -> bool{
    //ascending
    if values.windows(2).all(|w| w[0] < w[1] && (w[1] - w[0]) <= 3 && (w[1] - w[0]) >= 1){
        return true;
    }
    //descending
    if values.windows(2).all(|w| w[0] > w[1] && (w[0] - w[1]) <= 3 && (w[0] - w[1]) >= 1){
        return true;
    }
    false 
}

fn part_1(){
    let input = "input.txt";
    let lines = read_lines(&input);

    let mut valid_lines_count = 0;
    for (idx, line) in lines.iter().enumerate(){
        let line_split = line.split(" ");
        let values: Vec<i32> = line_split.map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();        
        
        if is_safe(&values){
            valid_lines_count += 1;
        }
    }

    println!("{}", valid_lines_count);
}

fn part_2(){
    let input = "input.txt";
    let lines = read_lines(&input);
}

fn main(){
    part_1();
    //part_2();
}