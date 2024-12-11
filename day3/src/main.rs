use std::fs;
use regex::Regex;

fn main() {
    let file_path = "input";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))").unwrap();

    let results: Vec<String> = re
        .captures_iter(&contents) // Borrow `contents`
        .filter_map(|caps| caps.get(0)) // Get the full match
        .map(|m| m.as_str().to_string()) // Convert to String
        .collect();

    println!("{:?}", results);

    let mut sum: u32 = 0;
    let mut doing = true;
    let mut _counter = 0;

    for i in 0..results.len() {
        if results[i] == "do()" {
            doing = true;
            _counter += 1;
        }
        else if results[i] == "don't()" {
            doing = false;
            _counter += 1;
        }
        else if doing {
            let numbers: Vec<_> = results[i][4..results[i].len() - 1].split(',').map(|n| n.trim().parse::<u32>().unwrap()).collect();
            sum += numbers[0] * numbers[1]
        }
    }
    println!("{:?}", sum)
}
