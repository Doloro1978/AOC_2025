use std::array;

const INPUT: &str = include_str!("./input.txt");

// this is a shitty way to do it, ik.
struct directions {
    top_left: bool,
    top: bool,
    top_right: bool,
    left: bool,
    right: bool,
    bottom_left: bool,
    bottom: bool,
    bottom_right: bool,
}

fn get_directions()

// true = @, false = .

fn main() {
    let input = INPUT.lines();
    let mut data: Vec<Vec<bool>> = Vec::new();
    input.for_each(|x| {
        let mut temp: Vec<bool> = Vec::new();
        x.chars()
            .for_each(|y| temp.push(if y == '@' { true } else { false }));
        data.push(temp);
    });
    
}
