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
        let mut buffer: Vec<i32> = Vec::with_capacity(12);
        for number in row {
            if buffer.is_empty() {
                buffer.push(number);
                continue;
            }
            let mut buffer_index = 0;
            // for x in buffer {}
            println!("{:?}", buffer[0]);
        }
    }
}
