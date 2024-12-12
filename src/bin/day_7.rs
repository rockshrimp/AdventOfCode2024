use advent_of_code_2024::read_lines;

fn apply_operator(solution_count: &mut i32, result: i64, operands: Vec<i64>){
    //println!("entering fn {} {:?}", result, operands);
    if result == 0{
        //println!("found a solution");
        *solution_count += 1;
        return;
    }else if operands.len() == 0 {
        //println!("no more operands");
        return;
    }
    else{
        if result % operands[0] == 0{
            //println!("in division {}, {:?}", result / operands[0], operands[1..].to_vec());
            apply_operator(solution_count, result / operands[0], operands[1..].to_vec());
        }
        if result - operands[0] >= 0{
            //println!("in substraction {}, {:?}", result / operands[0], operands[1..].to_vec());
            apply_operator(solution_count, result - operands[0], operands[1..].to_vec());
        }
    }
    return;
}

fn part_1(){
    let input = "input.txt";
    let lines: Vec<String> = read_lines(&input);
    
    let mut solution_count = 0;
    let mut calibration_result = 0;
    for line in lines{
        let colon_split = line.split(": ").collect::<Vec<&str>>();
        let result: i64 = colon_split[0].parse().unwrap();
        let operands = colon_split[1].split(" ").map(|v| v.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        //println!("{} {:?}", result, operands);
        
        let previous_count = solution_count;
        apply_operator(&mut solution_count, result, operands.into_iter().rev().collect::<Vec<i64>>());
        if solution_count != previous_count {
            calibration_result += result;
        }
        println!("{}", calibration_result);
    }
}

fn main(){
    part_1();
    //part_2();
}