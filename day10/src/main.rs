use std::fs;

fn main() {
    let file_path = "input";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    
    let table: Vec<Vec<u8>> = contents
        .lines()
        .into_iter()
        .map(|m| m
            .chars()
            .map(
                |c| c as u8 - b'0' as u8
            )
            .into_iter()
            .collect::<Vec<u8>>()
        )
        .collect();

    println!("{:?}", table);
    
    let mut sum = 0;
    for i in 0..table.len() {
        for j in 0..table[0].len() {
            if table[i][j] == 0 {
                let mut res = Vec::new();
                build_trail(&table, (i,j), 0, &mut res);
                println!("{:?}", res);
                sum += res.len()
            }
        }
    }

    println!("Result is {sum}")
}

fn build_trail(table: &Vec<Vec<u8>>, pos: (usize, usize), current_digit: u8, res: &mut Vec<(usize, usize)>) {
    if current_digit == 9 {
        res.push(pos);
    }
    if pos.0 < table.len() - 1 && table[pos.0+1][pos.1] == current_digit + 1 {
        build_trail(table, (pos.0+1, pos.1), current_digit + 1, res);
    }
    if  pos.0 > 0 && table[pos.0-1][pos.1] == current_digit + 1 {
        build_trail(table, (pos.0-1, pos.1), current_digit + 1, res);
    }
    if pos.1 < table.len() - 1 && table[pos.0][pos.1+1] == current_digit + 1 {
        build_trail(table, (pos.0, pos.1+1), current_digit + 1, res);
    }
    if pos.1 > 0 && table[pos.0][pos.1-1] == current_digit + 1 {
        build_trail(table, (pos.0, pos.1-1), current_digit + 1, res);
    }
}