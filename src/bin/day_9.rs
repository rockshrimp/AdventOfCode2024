use advent_of_code_2024::read_lines;

fn loop_condition(is_odd: bool, left:usize, right:usize) -> bool{
    if is_odd{
        return left < right;
    } else{
        return left <= right;
    }
}


fn part_1(){
    let input = "input.txt";
    let line: Vec<usize> = read_lines(&input)[0].chars().map(|c| c.to_digit(10).unwrap() as usize).collect();

    //println!("{:?}", line);

    let mut disk: Vec<i32> = Vec::new();    
    let empty_space = -1;

    for (idx, c) in line.iter().enumerate(){
        for _ in 0..*c{
            disk.push(if idx % 2 == 0{(idx / 2) as i32} else {empty_space});
        }
    }

    //println!("{:?}", disk);
    let disk_pretty = disk.iter().map(|v| {
        if *v != -1 {v.to_string()} else {".".to_string()}
    }
    ).collect::<Vec<String>>().join("");

    //println!("{:?}", disk_pretty);
    let mut empty_spaces = 0;
    let mut files_count = 0;

    for c in disk_pretty.chars(){
        if c.is_alphanumeric(){
            empty_spaces += 1;
        }else{
            files_count += 1
        }
    }

    //println!("{:?}", disk_pretty);
    //println!("{:?} files and {:?} empty spaces", files_count, empty_spaces);

    let mut left: usize = 1;
    let mut right: usize = disk.len() - 1;
    let mut sum:usize = 0;

    //let mut i =0;
    

    let is_disk_len_odd = disk.len() % 2 == 1;
    while loop_condition(is_disk_len_odd, left, right){
        if disk[left] != -1{
            //println!("adding {}*{} for index {}, iteration {}", left, disk[left], left, i);
            sum += left * disk[left] as usize;
            left += 1;
            //i += 1;

        }else{
            while disk[right] == -1{
                //println!("skipping empty right index {}", right);
                right -= 1;
            }
            //println!("adding {}*{} for index {} iteration {}", left, disk[right], right as usize, i);

            sum += left * disk[right] as usize;
            left += 1;
            right -= 1;
            //i += 1;

        }
        //println!("{} {} {}", sum, left, right);
    }

    println!("{}", sum);
}

fn main(){
    part_1();
}