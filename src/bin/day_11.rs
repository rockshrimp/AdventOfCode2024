use std::iter::successors;

use advent_of_code_2024::read_lines;


fn part_1(){
    let input = "input.txt";
    let mut stones: Vec<u64> = read_lines(&input)[0].split(" ").map(|v| v.parse().unwrap()).collect();

    for _ in 0..25{
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
        stones = new_stones;
    }

    println!("{:?}", stones.len());


}

fn main(){
    part_1();
    //part_2();
}