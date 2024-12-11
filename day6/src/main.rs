use std::fs;
use std::collections::{HashMap, HashSet};

type Position = (usize, usize);

fn main() {
    // Read the file and parse the grid
    let file_path = "input";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let lines: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();


    let mut loop_count = 0;

    let visited_positions = simulate_cycle(&lines).1;
    
    let mut unique_positions = HashSet::new();
    
    for &((i,j), _) in visited_positions.iter() {
        if !unique_positions.insert((i, j)) {
            continue;
        }

        let mut working_lines = lines.clone();

        if working_lines[i][j] == '.' || working_lines[i][j] == 'X' {
            working_lines[i][j] = '#';

            let result = simulate_cycle(&working_lines).0;
            if result != 0 {
                loop_count += result;
                
                working_lines[i][j] = '.';
            }
            

            working_lines[i][j] = '.';
        }
    }
    println!("Number of loops: {}", loop_count);
}

fn _print_lines(lines: &[Vec<char>]) {
    println!("--------------------------------------------------");
    for line in lines {
        println!("{}", line.iter().collect::<String>());
    }
}


fn simulate_cycle(working_lines: &Vec<Vec<char>>) -> (u32, HashSet<((usize, usize), char)>) {
    let mut loop_count = 0;
    let mut guard_position: Position = (0, 0);
    let mut current_char = ' ';

    // Find the initial guard position and direction
    for (i, row) in working_lines.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if ['^', '>', 'v', '<'].contains(&cell) {
                guard_position = (i, j);
                current_char = cell;
                break;
            }
        }
    }

    let original_position = guard_position;
    
    guard_position = original_position;
    let mut visited_positions:HashSet<(Position, char)> = HashSet::new();
    
    let directions: HashMap<char, (isize, isize)> = vec![
        ('^', (-1, 0)),
        ('>', (0, 1)),
        ('v', (1, 0)),
        ('<', (0, -1)),
    ]
    .into_iter()
    .collect();

    let turn_right: HashMap<char, char> = vec![
        ('^', '>'),
        ('>', 'v'),
        ('v', '<'),
        ('<', '^'),
    ]
    .into_iter()
    .collect();


    loop {
        if !visited_positions.insert((guard_position, current_char)) {
            loop_count += 1;
            break;
        }
        let (dx, dy) = directions[&current_char];
        let new_x = guard_position.0 as isize + dx;
        let new_y = guard_position.1 as isize + dy;

        // Check boundaries
        if new_x < 0
            || new_y < 0
            || new_x as usize >= working_lines.len()
            || new_y as usize >= working_lines[0].len()
        {
            break; // Stop movement if out of bounds or hitting an obstacle
        }
        if working_lines[new_x as usize][new_y as usize] == '#' {
            current_char = turn_right[&current_char];
            continue;
        }

        guard_position = (new_x as usize, new_y as usize);

    }
    (loop_count, visited_positions)
}
