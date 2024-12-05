use std::fs;

fn main() {
    let paths = [
        "inputs/example.txt",
        "inputs/example_2.txt",
        // "inputs/data.txt",
        // "inputs/corner-case.txt",
    ];

    let pattern = "XMAS";

    for path in paths {
        let content = fs::read_to_string(path).expect("Should have been able to read the file");
        let lines: Vec<&str> = content.split('\n').collect();

        let height = lines.len();
        let width = lines[0].len();

        for horizontal_line in &lines {
            let forward_count = count_occurrences_of_substring(horizontal_line, pattern, 0);
            let backward_count = count_occurrences_of_substring(
                &horizontal_line.chars().rev().collect::<String>(),
                pattern,
                0,
            );

            // println!("Line: {}", horizontal_line);
            // println!("Forward: {}, Backward: {}", forward_count, backward_count);
        }

        for column_number in 0..width {
            let downward_line = &lines
                .iter()
                .filter_map(|l| l.chars().nth(column_number))
                .collect::<String>();
            // println!("Downward lines: {}", downward_line);

            let upward_line = downward_line.chars().rev().collect::<String>();
            // println!("Upward lines: {}", upward_line);
        }

        if height >= width {
            for row in 0..(height + 1) {
                // println!("ROW{}", row);
                let mut diagonal = String::new();
                let mut curr_row = row;
                let mut curr_col = 0;

                while curr_row > 0 {
                    curr_row -= 1;
                    diagonal.push(lines[curr_row].chars().nth(curr_col).unwrap());
                    // println!("{}, {}", curr_row, curr_col);
                    curr_col += 1;
                }

                println!("{}", diagonal);
            }

            for col in (1..width).rev() {
                let mut diagonal = String::new();
                let mut curr_row = height - 1;
                let mut curr_col = col;

                while curr_col < width && curr_row > 0 {
                    diagonal.push(lines[curr_row].chars().nth(curr_col).unwrap());
                    curr_row -= 1;
                    curr_col += 1;
                }

                println!("{}", diagonal);
            }
        } else {
            for col in 0..(width + 1) {
                let mut diagonal = String::new();
                let mut curr_row = height - 1;
            }
        }

        // if height >= width {
        //     for row in 0..height {
        //         let mut up_diagonal_line = String::new();
        //         let mut curr_row = row;
        //         let mut curr_col: usize = 0;
        //         while curr_row >= 0 {
        //             up_diagonal_line.push(lines[row].chars().nth(curr_col).unwrap());
        //             curr_row -= 1;
        //             curr_col += 1;
        //         }
        //         println!("{}", up_diagonal_line);
        //     }
        // } else {
        //     for col in 0..width {
        //         let mut up_diagonal_line = String::new();
        //         let mut curr_col = col;
        //         let mut curr_row: usize = 0;
        //         while curr_col >= 0 {
        //             up_diagonal_line.push(lines[row].chars().nth(curr_col).unwrap());
        //             curr_col -= 1;
        //             curr_row += 1;
        //         }
        //         println!("{}", up_diagonal_line);
        //     }
        // }
    }
}

fn count_occurrences_of_substring(s: &str, substring: &str, curr_count: u32) -> u32 {
    match s.find(substring) {
        None => curr_count,
        Some(pos) => {
            let new_start = pos + substring.len();
            count_occurrences_of_substring(&s[new_start..], substring, curr_count + 1)
        }
    }
}
