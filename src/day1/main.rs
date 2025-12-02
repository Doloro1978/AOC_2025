const INPUT: &str = include_str!("./input.txt");

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount: i32,
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut result: Vec<Instruction> = Vec::new();
    let mut lines = input.lines();
    while let Some(x) = lines.next() {
        let mut chars = x.chars();
        let mut dir: Direction = Direction::Right;
        let mut amount: Vec<char> = Vec::new();
        if (chars.next() == Some('L')) {
            dir = Direction::Left;
        }
        while let Some(y) = chars.next() {
            amount.push(y);
        }
        result.push(Instruction {
            direction: dir,
            amount: amount.iter().collect::<String>().parse().unwrap(),
        })
    }
    result
}

fn main() {
    println!("{}", INPUT);
    let parsed_input: Vec<Instruction> = parse(INPUT);
    println!("{:#?}", parsed_input);
    let mut rotation: i32 = 50;
    let mut pass: i32 = 0;
    // The logic below has been rewrote 3 times for part 2 of the challange, i got the first answer
    // by accident.
    for x in &parsed_input {
        let mut amount = x.amount;
        let mut clicks = 0;
        match x.direction {
            Direction::Left => {
                for y in (0..=amount) {
                    if y == 0 {
                        continue;
                    }
                    rotation = rotation.abs();
                    rotation -= 1;
                    rotation = rotation % 100;
                    if rotation == 0 && y != amount {
                        clicks += 1
                    }
                    if rotation == -1 {
                        rotation = 99;
                    }
                }
            }
            Direction::Right => {
                for y in (0..=amount) {
                    if y == 0 {
                        continue;
                    }
                    rotation = rotation.abs();
                    rotation += 1;
                    rotation = rotation % 100;
                    if rotation == 0 && y != amount {
                        clicks += 1
                    }
                }
            }
        }
        if rotation == 0 {
            pass += 1;
        }
        pass += clicks;
    }
    println!("answer: {}", pass);
}
