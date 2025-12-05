use std::array;

const INPUT: &str = include_str!("./input.txt");
const PART2: bool = true;
const PART2_ARRAY_SIZE: usize = 12;

// I mentally gave up on this one, i fucked up the logic then kept trying to force it instead of
// rethinking it, it rlly killed my mental.

fn main() {
    let mut input = INPUT.lines();
    let mut data: Vec<Vec<i32>> = Vec::new();
    input.into_iter().for_each(|x| {
        let mut temp: Vec<i32> = Vec::new();
        x.chars().for_each(|y| {
            temp.push(unsafe { y.to_digit(10).unwrap_unchecked().try_into().unwrap() });
        });
        data.push(temp);
    });
    let mut result: Vec<i64> = Vec::new();
    for row in data {
        println!("");
        // There are no zero's in the input data
        let row_length: i32 = row.clone().len() as i32;
        let mut index: i32 = 0;
        let mut count = 1; // Which number we are aiming to get
        let mut amount = 12;
        let mut candidates: Vec<(i32, i32)> = Vec::new(); // Index, Number
        let mut temp: Vec<(i32, i32)> = Vec::new(); // Index, Number 
        for x in row.clone() {
            index += 1;
            temp.push((index, x));
        }
        let mut current_best: (i32, i32) = (0, 0);
        for z in 0..=amount {
            let mut temp_clone = temp.clone();
            let iter: Vec<&(i32, i32)> = temp_clone
                .iter()
                .filter(|(x)| !(x.0 > row_length - amount + z) && (x.0 > current_best.0))
                .collect();
            // println!("{:#?}", iter.clone());
            current_best = (0, 0);
            for x in iter {
                if x.1 > current_best.1 {
                    current_best = x.clone();
                }
            }
            candidates.push(current_best);
            // println!("{:?}", iter);
        }
        println!("{:?}", candidates);
        let mut best = 0;
        let mut temp: Vec<i64> = Vec::new();
        index = 0;
        for x in candidates.iter().rev() {
            index += 1;
            if index == 1 {
                temp.push(x.1.into());
                continue;
            }
            let meow: i64 = x.1 as i64;
            temp.push(meow * (10i64.pow((index - 1).try_into().unwrap())));
        }
        println!("combining");
        best = temp.iter().sum();
        println!("{:?}", best);
        result.push(best);
    }
    println!("{:?}", result);
    let final_result: i64 = result.iter().sum();
    println!("the fucking answer i hope: {:?}", final_result)
    // let mut final_result: u32 = result.iter().sum();
    // println!("{}", final_result);
}

fn sqrd(num: i32, by: i32) -> i64 {
    if by == 1 {
        return num.into();
    }
    let mut number = 0;
    for x in 1..=(by - 1) {
        if x == 1 {
            number = num * num;
            continue;
        }
        number = number * num;
    }
    println!("{} by {} = {}", num, by, number);
    return number.into();
}
