const INPUT: &str = include_str!("./input.txt");
const PART2: bool = true;

// I mentally gave up on this one, i fucked up the logic then kept trying to force it instead of
// rethinking it, it rlly killed my mental.

fn main() {
    let mut input = INPUT.lines();
    let mut data: Vec<Vec<u32>> = Vec::new();
    input.into_iter().for_each(|x| {
        let mut temp: Vec<u32> = Vec::new();
        x.chars().for_each(|y| {
            temp.push(unsafe { y.to_digit(10).unwrap_unchecked() });
        });
        data.push(temp);
    });
    let mut result: Vec<u32> = Vec::new();
    for row in data {
        println!("");
        print!("{:?} - ", row.clone());
        // There are no zero's in the input data
        let mut index = 0;
        let mut index_first = 0;
        let mut best_first = 0;
        let mut best_second = 0;
        let mut vec: Vec<u32> = Vec::new();
        for num in row.clone() {
            index += 1;
            let mut first = num * 10;
            for x in (index)..row.len() {
                let meow = row[x];
                vec.push(first + meow);
            }
        }
        let mut best = 0;
        for x in vec {
            if x > best {
                best = x;
            }
        }
        result.push(best);
    }
    let mut final_result: u32 = result.iter().sum();
    println!("{}", final_result);
}
