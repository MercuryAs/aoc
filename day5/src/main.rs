use std::fs;
use regex::Regex;

fn main() {
    let file_path = "input";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let re1: Regex = Regex::new(r"(\d+)\|(\d+)").unwrap();

    let ordering: Vec<(u32, u32)> = re1
        .captures_iter(&contents)
        .map(|m| (m[1].parse::<u32>().unwrap(), m[2].parse::<u32>().unwrap()))
        .collect();

    let re2: Regex = Regex::new(r"(\d+(?:,\d+)+)").unwrap();

    let mut updates: Vec<Vec<u32>> = re2
        .captures_iter(&contents)
        .map(|m| m[0].split(',').map(|s| s.parse::<u32>().unwrap()).collect())
        .collect();

    let mut valid_middle_pages:Vec <u32> = vec![];
    for i in 0..updates.len() {
        let mut valid = false;
        for j in 0..updates[i].len() {
            for k in j+1..updates[i].len() {
                if !ordering.contains(&(updates[i][j], updates[i][k])) {
                    valid = true;
                    (updates[i][j], updates[i][k]) = (updates[i][k], updates[i][j]);
                }
            }
        }
        if valid {
            valid_middle_pages.push(updates[i][updates[i].len()/2]);
        }
    }

    println!("{}", valid_middle_pages.iter().sum::<u32>());
}