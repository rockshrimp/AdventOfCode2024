#![allow(dead_code)]
#![allow(unused_variables)]

use regex::Regex;
use advent_of_code_2024::read_lines;

fn part_1(){
    let input = "input.txt";
    let lines = read_lines(&input);
    
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut result = 0;
    for line in lines{
        let matches: Vec<_> = re.find_iter(&line).map(|m| m.as_str()).collect();   

        let mut line_sum = 0;
        for val in matches{
            //println!("{}", &val[4..val.len() - 1]);
            let line_split = &val[4..val.len() - 1]
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
            line_sum += line_split[0] * line_split[1];
        }
        result += line_sum;
    }

    println!("{}", result);
}

fn part_2(){
    let input = "input.txt";
    let lines = read_lines(&input);

    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let mut result = 0;
    let mut enabled = true;

    for line in lines{
        let matches: Vec<_> = re.find_iter(&line).map(|m| m.as_str()).collect();   
        let mut line_sum = 0;
  
        for val in matches{
            match val{
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    if enabled{
                        let line_split = &val[4..val.len() - 1]
                        .split(",")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                        line_sum += line_split[0] * line_split[1];
                    }
                }
            }
        }
        result += line_sum;
    }

    println!("{}", result);

}

fn main(){
    //part_1();
    part_2();
}