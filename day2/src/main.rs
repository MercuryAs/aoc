use std::fs;

fn main() {
    let file_path = "input";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    

    let mut sum = 0;
    
    for line in contents.lines() {
        let numbers: Vec<i32> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

        let safe = is_safe(&numbers);
        if !safe {
            for i in 0..numbers.len() {
                let mut arr = numbers.clone();
                arr.remove(i);
                if is_safe(&arr) {
                    sum += 1;
                    break
                }
            }
            
        }

        else {
            sum += 1;
        }
    }

    println!("Sum is {}", sum)
}

fn is_safe(numbers: &[i32]) -> bool {
    let mut safe = true;

    if numbers[1] < numbers[0] {
        for i in 0..numbers.len()-1 {
            if numbers[i+1] >= numbers[i] {
                println!("Décroissant puis croissant ou constant");
                safe = false;
            }
            else if numbers[i+1] < numbers[i] - 3 {
                println!("Différence de {}", numbers[i+1] - numbers[i]);
                safe = false;
            }
        }
    }
    else if numbers[1] > numbers[0]{
        for i in 0..numbers.len()-1 {
            if numbers[i+1] <= numbers[i]{
                println!("Croissant puis décroissant ou constant");
                safe = false;
            }
            else if numbers[i+1] > numbers[i] + 3 {
                println!("Différence de {}", numbers[i+1] - numbers[i]);
                safe = false;
            }
        }
    }
    else {
        safe = false;
    }

    safe
}
