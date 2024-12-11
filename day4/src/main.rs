use std::fs;

fn main() {
    let file_path = "input";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let lines: Vec<Vec<char>> = contents.lines().map(|line| line.trim().chars().collect()).collect();
    let mut counter = 0;

    for i in 1..lines.len() - 1 {
        for j in 1..lines[0].len() - 1 {
            if lines[i][j] == 'A' {
                if lines[i+1][j+1] != lines[i-1][j-1] && lines[i+1][j-1] != lines[i-1][j+1] && ['M','S'].contains(&lines[i+1][j+1]) && ['M','S'].contains(&lines[i-1][j+1]) && ['M','S'].contains(&lines[i+1][j-1]) && ['M','S'].contains(&lines[i-1][j-1]) {
                    println!("A l'endroit, i: {}, j: {}", i, j);
                    counter += 1
                }
            }
        }
    }


    // for i in 0..lines.len() {
    //     for j in 0..lines[0].len() {
    //         if lines[i][j] == 'X' {
    //             if j < lines[i].len() - 3 {
    //                 if lines[i][j+1] == 'M' && lines[i][j+2] == 'A' && lines[i][j+3] == 'S' {
    //                     println!("A l'endroit, i: {}, j: {}", i, j);
    //                     counter += 1
    //                 }
    //             }
    //             if j > 2 {
    //                 if lines[i][j-1] == 'M' && lines[i][j-2] == 'A' && lines[i][j-3] == 'S' {
    //                     println!("A l'envers, i: {}, j: {}", i, j);
    //                     counter += 1
    //                 }
    //             }
    //             if i < lines.len() - 3 {
    //                 if lines[i+1][j] == 'M' && lines[i+2][j] == 'A' && lines[i+3][j] == 'S' {
    //                     println!("A la verticale, i: {}, j: {}", i, j);
    //                     counter += 1
    //                 }
    //             }
    //             if i > 2 {
    //                 if lines[i-1][j] == 'M' && lines[i-2][j] == 'A' && lines[i-3][j] == 'S' {
    //                     println!("A la verticale et à l'envers, i: {}, j: {}", i, j);
    //                     counter += 1
    //                 }
    //             }
    //             if j < lines[i].len() - 3 && i < lines.len() - 3 {
    //                 if lines[i+1][j+1] == 'M' && lines[i+2][j+2] == 'A' && lines[i+3][j+3] == 'S' {
    //                     println!("Diag bas droite, i: {}, j: {}", i, j);
    //                     counter += 1
    //                 }
    //             }
    //             if j < lines[i].len() - 3 && i > 2 {
    //                 if lines[i-1][j+1] == 'M' && lines[i-2][j+2] == 'A' && lines[i-3][j+3] == 'S' {
    //                     println!("Diag haut droite, i: {}, j: {}", i, j);
    //                     counter += 1
    //                 }
    //             }
    //             if j > 2 && i < lines.len() - 3 {
    //                 if lines[i+1][j-1] == 'M' && lines[i+2][j-2] == 'A' && lines[i+3][j-3] == 'S' {
    //                     println!("Diag bas gauche, i: {}, j: {}", i, j);
    //                     counter += 1
    //                 }
    //             }
    //             if j > 2 && i > 2 {
    //                 if lines[i-1][j-1] == 'M' && lines[i-2][j-2] == 'A' && lines[i-3][j-3] == 'S' {
    //                     println!("Diag haut gauche, i: {}, j: {}", i, j);
    //                     counter += 1
    //                 }
    //             }
    //         }
    //     }
    // }
    println!("{}", &counter);
}