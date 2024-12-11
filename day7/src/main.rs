use std::{collections::VecDeque, fs, result};
use regex::Regex;


fn main() {
    println!("{}", concat!(4, 5));
    let file_path = "input";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let re: Regex = Regex::new(r"(\d+):( \d+)+").unwrap();

    let results: Vec<(u64, VecDeque<u128>)> = re.captures_iter(&contents)
        .map(|m| {
            let id = m.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let values:VecDeque<u128> = m.get(0).unwrap().as_str().split_whitespace().skip(1).map(|s| s.parse::<u128>().unwrap()).collect();
            (id, values)
        })
        .collect();

    let mut sum = 0;
    for (id, values) in results {
        if recursive_summing(id, values, &mut vec![]).contains(&id) {
            sum += id;
        }
    }

    println!("{sum}")
}

fn recursive_summing(id: u64,mut values: VecDeque<u128>, result: &mut Vec<u64>) -> &Vec<u64> {
    if values.len() > 1 {
        let val1 = values.pop_front().unwrap();
        let val2 = values.pop_front().unwrap();
        let mut list1 = values.clone();
        let mut list2 = values.clone();
        let mut list3 = values.clone();
        
        list1.push_front(val1 + val2);
        list2.push_front(val1 * val2);
        let val = concat_s(val1, val2);
        list3.push_front(val);


        recursive_summing(id, list1, result);
        recursive_summing(id, list2, result);
        recursive_summing(id, list3, result);
    } else {
        if id == *values.get(0).unwrap() as u64 {
            result.push(id);
        }
    }
    result
}

fn concat_s(a: u128, b: u128) -> u128 {
    (a.to_string() + &b.to_string()).parse().unwrap()
}
