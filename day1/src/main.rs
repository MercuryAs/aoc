use std::fs;
use find_all::FindAll;

fn main() {
    let file_path = "input";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in contents.lines() {
        let numbers: Vec<u32> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

        list1.push(numbers[0]);
        list2.push(numbers[1]);
    }

    list1.sort();
    list2.sort();

    let mut sum_diff = 0;
    let mut sum_sim = 0;

    for i in 0..list1.len() {
        let val = list2[i].abs_diff(list1[i]);
        sum_diff += val;
        
        let repetitions_in_l2 = list2.iter().find_all(|num: &&u32| **num == list1[i]);

        match repetitions_in_l2 {
            Some(x) => sum_sim += (x.len() as u32) * list1[i],
            None => sum_sim += 0
        }
    }

    println!("Sum of differences is {}", sum_diff);
    println!("Sum of similarities is {}", sum_sim);
}
