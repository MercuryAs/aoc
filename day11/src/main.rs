use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("input").expect("Error reading file");

    let result: u64 = contents
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .iter()
        .map(|&v| {
            let mut local_cache = HashMap::new(); // Each thread gets its own cache
            rec_blink(v, 0, 75, &mut local_cache)
        })
        .sum();

    println!("{result}");
}

fn is_of_even_length(number: &u64) -> bool {
    number.to_string().len() % 2 == 0
}


fn split_even_length(number: &u64) -> (u64, u64) {
    let num_str = number.to_string();
    let l = num_str.len();
    
    let t = num_str.split_at(l/2);
    (t.0.parse().unwrap(), t.1.parse().unwrap())
}

fn rec_blink(
    num: u64,
    n: u8,
    max: u8,
    cache: &mut HashMap<(u64, u8), u64>,
) -> u64 {
    if n >= max {
        return 1;
    }

    if let Some(&cached_result) = cache.get(&(num, n)) {
        return cached_result;
    }

    let result = match num {
        0 => rec_blink(1, n + 1, max, cache),
        num if is_of_even_length(&num) => {
            let (left, right) = split_even_length(&num);
            rec_blink(left, n + 1, max, cache)
                + rec_blink(right, n + 1, max, cache)
        }
        _ => rec_blink(num * 2024, n + 1, max, cache),
    };

    cache.insert((num, n), result);

    result
}