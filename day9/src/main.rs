use std::fs;

fn main() {
    let file_path = "input";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    
    let mut res = Vec::new();
    for (i, c) in contents.chars().enumerate() {
        let n = c as usize - '0' as usize;
        if i%2 == 0 {
            res.push(vec![(Some(i/2), false); n]);
        } else {
            res.push(vec![(None, false); n]);
        }
    }


    
    fn part1(res: &mut Vec<Option<usize>>) {
        let mut i = 1;
        while _has_pattern(&res) {
            let n = res.len()-i;
            let c = res[n];
            res[n] = None;
            i += 1;

            for j in 0..res.len() {
                if res[j] == None {
                    res[j] = c;
                    break;
                }
            }
        }
    }


    fn part2(res: &mut Vec<Vec<(Option<usize>, bool)>>) {
        for i in (0..res.len()).rev() {
            if !res[i].contains(&(None, false)) {
                for j in 1..i {
                    if res[i].len()>0 && res[i].len() <= res[j].len() && res[j].contains(&(None, false)){
                        let new_elems= (vec![(res[i][0].0, true); res[i].len()], vec![(None, false); res[j].len() - res[i].len()]);
                        if res[j].contains(&(None, false)) {
                            res.remove(j);
                        }
                        res.insert(j, new_elems.0);
                        res.remove(i);
                        if new_elems.1.len() >0 {
                            res.insert(j+1, new_elems.1);
                            if res[j+2].contains(&(None, false)){
                                res[j+1] = [res[j+2].clone(), res[j+1].clone()].concat();
                            }
                            if res[j].contains(&(None, false)){
                                res[j+1] = [res[j].clone(), res[j+1].clone()].concat()
                            }
                        }
                        res.insert(i, vec![(None, false); res[j].len()]);
                        break;
                    }
                }
            }
        }
    }

    part2(&mut res);
    print_res(&mut res);

    // let mut sum: u128 = 0;
    // for i in 0..res.len() {
    //     if let Some(x) = res[i] {
    //         sum += x as u128 * i as u128;
    //     } else {
    //         continue;
    //     }
    // }

    // println!("{sum}")
}


fn _has_pattern(res: &[Option<usize>]) -> bool {
    let mut found_digit = false;
    let mut found_dot = false;

    for &x in res {
        match x {
            Some(_) if found_dot => return true, // Pattern matched: digit -> dot -> digit
            Some(_) => found_digit = true,      // Start of a digit sequence
            None if found_digit => found_dot = true, // Dot after digit
            _ => {
                found_digit = false; // Reset when pattern breaks
                found_dot = false;
            }
        }
    }
    false
}

fn print_res(res: &mut Vec<Vec<(Option<usize>, bool)>>){
    let mut sum = 0;
    let mut counter = 0;
    for vec in res {
        for (x,_) in vec {
            match x {
                Some(num) => {
                    sum += counter * *num;
                    print!("{}", num);
                }
                None => print!("."),
            }
            counter += 1;
        }
    }
    println!();
    println!("{sum}")
}

