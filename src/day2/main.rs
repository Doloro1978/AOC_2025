const INPUT: &str = include_str!("./input.txt");
const PART2: bool = true;

#[derive(Debug)]
struct ProductRange {
    first: i64,
    last: i64,
}

fn main() {
    let mut product_ranges = INPUT.split(",");
    let mut product_ids: Vec<ProductRange> = Vec::new();
    let mut invalid_ids: Vec<i64> = Vec::new();

    while let Some(range) = product_ranges.next() {
        let split_range = range.split_once("-").unwrap();
        let x = split_range.1.replace("\n", "");
        product_ids.push(ProductRange {
            first: split_range.0.parse().unwrap(),
            last: x.parse().unwrap(),
        });
    }
    for range in product_ids {
        for x in range.first..=range.last {
            let mut string = x.to_string();
            if !PART2 {
                if (string.len() % 2) == 0 {
                    let string2 = string.split_off(string.len() / 2);
                    if string == string2 {
                        invalid_ids.push(x);
                        continue;
                    }
                }
            } else {
                //Part 2, More rules
                let mut chars = string.chars();
                for x in 1..=string.len() {
                    if x == string.len() {
                        continue;
                    }
                    let string1 = string.get(0..x).unwrap();
                    // println!("{:#?}", string1);
                    let string2 = string.replace(string1, "");
                    if string2.len() == 0 {
                        // println!(
                        //     "found: {}, it contains {}, {} times",
                        //     string,
                        //     string1,
                        //     (string.len() / string1.len())
                        // );
                        invalid_ids.push(string.parse().unwrap());
                        break;
                    }
                }
            }
        }
    }
    let mut final_result: i64 = 0;
    // println!("{:#?}", invalid_ids);
    for y in invalid_ids {
        final_result += y;
    }
    println!("{:#?}", final_result);
}
