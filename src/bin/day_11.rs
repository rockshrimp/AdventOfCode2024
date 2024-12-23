use std::{iter::successors, result};

use std::collections::HashMap;

use advent_of_code_2024::read_lines;
use itertools::Itertools;

fn blink(stones: Vec<u64>) -> Vec<u64>{
    let mut new_stones = Vec::new();
    for stone in stones{
        if stone == 0{
            new_stones.push(1);
        }else{
            let digit_count = successors(Some(stone), |&n| (n >= 10).then(|| n / 10)).count();
            if digit_count % 2 == 0{
                
                let right_stone = stone % u64::pow(10, (digit_count / 2) as u32);
                let left_stone = (stone - right_stone) / u64::pow(10, (digit_count / 2) as u32) as u64;
                
                //println!("{:?}", vec![stone, right_stone, left_stone]);
                new_stones.push(left_stone);
                new_stones.push(right_stone);
            }else{
                new_stones.push(stone * 2024);
            }
        }
    }
    return new_stones;
}

fn part_1(){
    let input = "input.txt";
    let mut stones: Vec<u64> = read_lines(&input)[0].split(" ").map(|v| v.parse().unwrap()).collect();

    for _ in 0..25{
        stones = blink(stones);
    }
    println!("{:?}", stones.len());
}


fn blink_2(stones: HashMap<u64, usize>) -> HashMap<u64, usize>{
    let mut new_stones: HashMap<u64, usize> = HashMap::new();

    for (stone, count) in stones.iter(){
        if *stone == 0{
            *new_stones.entry(1).or_insert(0) += count;
        }else{
            let digit_count = successors(Some(*stone), |&n| (n >= 10).then(|| n / 10)).count();
            if digit_count % 2 == 0{
                
                let right_stone = stone % u64::pow(10, (digit_count / 2) as u32);
                let left_stone = (stone - right_stone) / u64::pow(10, (digit_count / 2) as u32) as u64;
                
                *new_stones.entry(left_stone).or_insert(0) += count;
                *new_stones.entry(right_stone).or_insert(0) += count;
            }else{
                *new_stones.entry(stone * 2024).or_insert(0) += count;
            }
        }
    }
    return new_stones;
}

fn part_2(){
    let input = "input.txt";
    let mut stones: HashMap<u64, usize> = read_lines(&input)[0].split(" ").map(|v| v.parse().unwrap()).counts();
    for i in 0..75{
        stones = blink_2(stones);
    } 
    println!("{}", stones.values().sum::<usize>());
}


fn main(){
    //part_1();
    part_2();
}