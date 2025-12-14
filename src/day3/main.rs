use std::array;
use std::rc::Rc;

const INPUT: &str = include_str!("./input.txt");
const PART2: bool = true;
const PART2_ARRAY_SIZE: usize = 12;

// I mentally gave up on this one, i fucked up the logic then kept trying to force it instead of
// rethinking it, it rlly killed my mental.

fn main() {
    let mut input = INPUT.lines();
    let mut data: Vec<Vec<i64>> = Vec::new();
    input.into_iter().for_each(|x| {
        let mut temp: Vec<i64> = Vec::new();
        x.chars().for_each(|y| {
            temp.push(unsafe { y.to_digit(10).unwrap_unchecked().try_into().unwrap() });
        });
        data.push(temp);
    });
    let mut result: Vec<i64> = Vec::new();
    for row in data {
        let mut buffer: [i64; 12] = [-1; 12];
        let mut is_first = true;
        let mut index_length: (i32, i32) = (0, row.len() as i32); // Index and length 
        for number in row {
            if is_first {
                buffer[0] = number;
                is_first = false;
                continue;
            }
            let mut buffer_index = 0;
            let mut wipe = false;
            for x in buffer.clone().iter().cloned() {
                let space = i32::is_negative((index_length.1 - index_length.0) - 3); // pos if
                // there isnt
                // space
                if wipe && !space {
                    buffer[buffer_index] = -1;
                    buffer_index += 1;
                    continue;
                }
                if space {
                    if x == -1 {
                        buffer[buffer_index] = number;
                    }
                }
                if number > x && !space {
                    buffer[buffer_index] = number;
                    wipe = true;
                }
                buffer_index += 1;
            }
            index_length.0 += 1;
        }
        println!("{:?}", buffer);
        buffer.reverse();
        for x in 0..=(buffer.len() - 1) {
            buffer[x] = buffer[x] * 10i64.pow(x.try_into().unwrap());
        }
        println!("{:?}", buffer.iter().sum::<i64>());
        result.push(buffer.iter().sum::<i64>());
    }
    println!("resalt: {}", result.iter().sum::<i64>());
}
