use std::fs::read_to_string;
use regex::Regex;

fn solve(button_a: (isize, isize), button_b: (isize, isize), prize: (isize, isize)) -> Option<(isize, isize)> {
    let b_count = (button_a.0 * prize.1 - button_a.1 * prize.0) / (button_b.1 * button_a.0 - button_b.0 * button_a.1);
    let rem_b = (button_a.0 * prize.1 - button_a.1 * prize.0) % (button_b.1 * button_a.0 - button_b.0 * button_a.1);
    let a_count = (prize.0 - b_count * button_b.0) / button_a.0;
    let rem_a = (prize.0 - b_count * button_b.0) % button_a.0;
    if rem_b != 0 || rem_a != 0 {
        return None;
    };
    Some((a_count, b_count))
}

fn part_1(input: &str) -> isize{ 
    let re: Regex = Regex::new(r"X.(\d+), Y.(\d+)").unwrap();

    let mut button_a: (isize, isize) = (0, 0);
    let mut button_b: (isize, isize) = (0, 0);
    let mut prize: (isize, isize) = (0, 0);

    let mut token_count: isize = 0;

    for (idx, (_, [x, y])) in re.captures_iter(input).map(|c| c.extract()).enumerate() {
        let values = (x.parse().unwrap(), y.parse().unwrap());
        //println!("{:?}", values);
        match idx % 3{
            0 => button_a = values,
            1 => button_b = values,
            2 => prize = values,
            _ => ()
        }

        if idx % 3 == 2 {
            match solve(button_a, button_b, prize){
                Some((a_count, b_count)) =>{
                    //println!("a: {} b: {}", a_count, b_count);
                    token_count += a_count * 3 + b_count;
                },
                _ => ()
            }
            //println!("{}", token_count);
        }
    }
    return token_count;
}


fn part_2(input: &str) -> isize{ 
    let re: Regex = Regex::new(r"X.(\d+), Y.(\d+)").unwrap();

    let mut button_a: (isize, isize) = (0, 0);
    let mut button_b: (isize, isize) = (0, 0);
    let mut prize: (isize, isize) = (0, 0);

    let mut token_count: isize = 0;

    for (idx, (_, [x, y])) in re.captures_iter(input).map(|c| c.extract()).enumerate() {
        let values = (x.parse().unwrap(), y.parse().unwrap());
        //println!("{:?}", values);
        match idx % 3{
            0 => button_a = values,
            1 => button_b = values,
            2 => prize = (values.0 + 10000000000000, values.1 + 10000000000000),
            _ => ()
        }

        if idx % 3 == 2 {
            match solve(button_a, button_b, prize){
                Some((a_count, b_count)) =>{
                    //println!("a: {} b: {}", a_count, b_count);
                    token_count += a_count * 3 + b_count;
                },
                _ => ()
            }
            //println!("{}", token_count);
        }
    }
    return token_count;
}


fn main(){
    let filename = "input.txt";
    let input = read_to_string(filename).unwrap();   

    println!("{}", part_1(input.as_str()));
    println!("{}", part_2(input.as_str()));
}

#[cfg(test)]
mod tests {
    use crate::part_1;

    const TEST: &str = 
"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

     #[test]
    fn test_part_1_1() {
        assert_eq!(part_1(TEST), 480);
    }
}