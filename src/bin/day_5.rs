use advent_of_code_2024::read_lines;
use std::collections::HashMap;

fn parse_input(lines: Vec<String>, rules: &mut HashMap<String, Vec<String>>, updates:  &mut Vec<Vec<String>>){
    let mut rules_lines = true;
    for line in lines{
        if line == ""{
            rules_lines = false;
            continue;
        }
        if rules_lines{
            let rule_vec: Vec<String> = line.split("|").map(|s| s.to_string()).collect();

            let before = rule_vec[1].clone();
            let after = rule_vec[0].clone(); 

            rules.entry(before)
                .or_insert_with(Vec::new) // If the key doesn't exist, create a new empty vector
                .push(after);
        }else{
            updates.push(line.split(",").map(|s| s.to_string()).collect());
        }
    }
}

fn part_1(){
    let input = "input.txt";
    let lines = read_lines(&input);

    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    let mut updates: Vec<Vec<String>> = Vec::new();

    parse_input(lines, &mut rules, &mut updates);

    let mut middle_pages = Vec::new();
    for update in updates{
        //println!("{:?}", update);

        let mut update_impossible = false;
        for (i, page) in update[.. update.len() - 1].iter().enumerate(){
            if rules.get(page).is_none(){
                continue;
            }else{
                for rule in rules.get(page).unwrap(){
                    if update[i + 1..].contains(rule){
                        //println!("cannot update because {} is before {}", rule, page);
                        update_impossible = true;
                        break;
                    }
                }
            }
            if update_impossible{
                break;
            }
        }
        if !update_impossible{
            let middle_index: usize = update.len() / 2;
            middle_pages.push(update[middle_index].parse::<i32>().unwrap());
        }
    }

    println!("{:?}", middle_pages.iter().sum::<i32>());
}

fn rearrange_update(update: Vec<String>, rules: &HashMap<String, Vec<String>>) -> Vec<String>{
    let mut rearranged_update: Vec<String> = update;

    for i in 0..rearranged_update.len(){
        for _ in i + 1..rearranged_update.len(){
            if rules.get(&rearranged_update[i]).is_some(){
                // Swap element at index i with the one that should be before it
                for rule in rules.get(&rearranged_update[i]).unwrap(){
                    if rearranged_update[i..].contains(rule){
                        let dependency_idx = rearranged_update.iter().position(|r| r == rule).unwrap();
                        rearranged_update.swap(i, dependency_idx);
                    }
                }
            }else {
                break;
            }
        }
    }
    //println!("{:?} rearranged_update", rearranged_update);
    return rearranged_update;
}

fn part_2(){
    let input = "input.txt";
    let lines = read_lines(&input);

    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    let mut updates: Vec<Vec<String>> = Vec::new();

    parse_input(lines, &mut rules, &mut updates);

    let mut middle_pages = Vec::new();
    for update in updates{
        //println!("{:?}", update);

        let mut update_impossible = false;
        for (i, page) in update[.. update.len() - 1].iter().enumerate(){
            if rules.get(page).is_none(){
                continue;
            }else{
                for rule in rules.get(page).unwrap(){
                    if update[i + 1..].contains(rule){
                        //println!("{:?} cannot update because {} is before {}", update, rule, page);
                        update_impossible = true;
                        break;
                    }
                }
            }
            if update_impossible{
                break;
            }
        }
        if update_impossible{
            let rearranged_update = rearrange_update(update, &rules);
            let middle_index: usize = rearranged_update.len() / 2;
            middle_pages.push(rearranged_update[middle_index].parse::<i32>().unwrap());
            
        }
    }
    println!("{:?}", middle_pages.iter().sum::<i32>());
}

fn main(){
    //part_1();
    part_2();
}