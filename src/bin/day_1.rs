#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::read_to_string;
use std::collections::HashMap;

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


fn part_2(){
    let input = "input.txt";
    let lines = read_lines(&input);
    let mut left = HashMap::<i32, i32>::new();
    let mut right = HashMap::<i32, i32>::new();
    for line in lines.iter(){
        let mut line_split = line.split("   ");
        let left_val = line_split.next().unwrap().parse::<i32>().unwrap();
        let right_val = line_split.next().unwrap().parse::<i32>().unwrap();

        match left.get(&left_val){
            Some(val) => left.insert(left_val, val + 1),
            None => left.insert(left_val, 1),
        };

        match right.get(&right_val){
            Some(val) => right.insert(right_val, val + 1),
            None => right.insert(right_val, 1),
        };
        
    }

    let mut sum = 0;
    for (key, val) in left.iter() {
        match right.get(key){
            Some(v) => sum += key * val * v,
            _ => (),
        };
    }

    println!("{}", sum);
}

fn main(){
    //part_1();
    part_2();
}