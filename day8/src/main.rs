use std::{collections::HashMap, fs};
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let file_path = "input";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let mut lines: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

        let re: Regex = Regex::new(r"(\w)").unwrap();
        let mut char_positions: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();
    
        for (line_idx, line) in contents.lines().enumerate() {
            for mat in re.find_iter(line) {
                let c = mat.as_str().chars().next().unwrap();
                let pos = (line_idx, mat.start());
                char_positions.entry(c).or_insert_with(HashSet::new).insert(pos);
            }
        }


    let mut counter = 0;

    for c in char_positions {
        let positions = c.1;
        for pos1 in &positions {
            for pos2 in &positions {
                if pos1 == pos2 {
                    continue
                }
                let dx: isize = pos1.0 as isize - pos2.0 as isize;
                let dy: isize = pos1.1 as isize - pos2.1 as isize;
                let mut i = 0;
                while !((pos1.0 as isize + i * dx) < 0
                    || (pos1.0 as isize + i * dx) as usize > lines.len() - 1
                    || (pos1.1 as isize + i * dy) < 0
                    || (pos1.1 as isize + i * dy) as usize > lines[0].len() - 1) {
                        let x_diff = i * dx;
                        let y_diff = i * dy;
                        i += 1;

                        if lines[(pos1.0 as isize + x_diff) as usize][(pos1.1 as isize + y_diff) as usize] != '#' {
                            counter += 1;
                        }
                        lines[(pos1.0 as isize + x_diff) as usize][(pos1.1 as isize + y_diff) as usize] = '#';
                    }

            }
        }
    }
    // _print_lines(lines);
    println!("{counter}")
    
}

fn _print_lines(lines: Vec<Vec<char>>) {
    for l in lines{
        println!("{}", l.iter().collect::<String>())
    }
}
