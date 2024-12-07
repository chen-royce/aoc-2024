use std::fs;

fn main() {
    part_2();
}

fn part_2() {
    let paths = [
        "inputs/example.txt",
        "inputs/data.txt",
        // "inputs/corner-case.txt",
    ];

    for path in paths {
        let content = fs::read_to_string(path).expect("Should have been able to read the file");
        let lines: Vec<Vec<char>> = content
            .split('\n')
            .map(|line| line.chars().collect())
            .collect();

        let num_rows = lines.len();
        let num_cols = lines[0].len();

        let mut count: u32 = 0;

        for r in 1..num_rows - 1 {
            for c in 1..num_cols - 1 {
                if lines[r][c] == 'A' && check_part_2_neighbors(&lines, r, c) {
                    count += 1;
                }
            }
        }

        println!("{}", count);
    }
}

fn check_part_2_neighbors(input: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    match (
        input[row + 1][col + 1],
        input[row - 1][col - 1],
        input[row + 1][col - 1],
        input[row - 1][col + 1],
    ) {
        ('M', 'S', 'M', 'S') => true,
        ('S', 'M', 'M', 'S') => true,
        ('M', 'S', 'S', 'M') => true,
        ('S', 'M', 'S', 'M') => true,
        _ => false,
    }
}

fn part_1() {
    let paths = [
        "inputs/example.txt",
        "inputs/data.txt",
        // "inputs/corner-case.txt",
    ];

    let pattern = "XMAS";

    for path in paths {
        let content = fs::read_to_string(path).expect("Should have been able to read the file");
        let lines: Vec<&str> = content.split('\n').collect();

        let height = lines.len();
        let width = lines[0].len();

        let mut total_count = 0;

        // HORIZONTAL
        for horizontal_line in &lines {
            let forward_count = count_occurrences_of_substring(horizontal_line, pattern, 0);
            let backward_count = count_occurrences_of_substring(
                &horizontal_line.chars().rev().collect::<String>(),
                pattern,
                0,
            );

            total_count += forward_count + backward_count;
        }

        // VERTICAL
        for column_number in 0..width {
            let downward_line = &lines
                .iter()
                .filter_map(|l| l.chars().nth(column_number))
                .collect::<String>();
            // println!("Downward lines: {}", downward_line);

            let upward_line = downward_line.chars().rev().collect::<String>();
            // println!("Upward lines: {}", upward_line);

            let downward_count = count_occurrences_of_substring(&downward_line, pattern, 0);
            let upward_count = count_occurrences_of_substring(&upward_line, pattern, 0);

            total_count += downward_count;
            total_count += upward_count;
        }

        // DIAGONAL (DOWN AND RIGHT)
        // Lower-left half
        for row in 0..height {
            let mut diagonal = String::new();
            let mut curr_row = row;
            let mut curr_col = 0;

            while curr_row < height && curr_col < width {
                diagonal.push(lines[curr_row].chars().nth(curr_col).unwrap());
                curr_row += 1;
                curr_col += 1;
            }

            let reverse_diagonal = diagonal.chars().rev().collect::<String>();

            let diagonal_count_1 = count_occurrences_of_substring(&diagonal, pattern, 0);
            let diagonal_count_2 = count_occurrences_of_substring(&reverse_diagonal, pattern, 0);

            total_count += diagonal_count_1;
            total_count += diagonal_count_2;
        }

        // Upper-right half
        for col in 1..width {
            let mut diagonal = String::new();
            let mut curr_row = 0;
            let mut curr_col = col;

            while curr_row < height && curr_col < width {
                diagonal.push(lines[curr_row].chars().nth(curr_col).unwrap());
                curr_row += 1;
                curr_col += 1;
            }

            let reverse_diagonal = diagonal.chars().rev().collect::<String>();

            let diagonal_count_1 = count_occurrences_of_substring(&diagonal, pattern, 0);
            let diagonal_count_2 = count_occurrences_of_substring(&reverse_diagonal, pattern, 0);

            total_count += diagonal_count_1;
            total_count += diagonal_count_2;
        }

        // DIAGONAL (DOWN AND LEFT)
        // Bottom-right half
        for row in 0..height {
            let mut diagonal = String::new();
            let mut curr_row = row;
            let mut curr_col = width - 1;

            while curr_row < height {
                diagonal.push(lines[curr_row].chars().nth(curr_col).unwrap());
                curr_row += 1;
                curr_col = if curr_col > 0 {
                    curr_col - 1
                } else {
                    break;
                };
            }

            let reverse_diagonal = diagonal.chars().rev().collect::<String>();

            let diagonal_count_1 = count_occurrences_of_substring(&diagonal, pattern, 0);
            let diagonal_count_2 = count_occurrences_of_substring(&reverse_diagonal, pattern, 0);

            total_count += diagonal_count_1;
            total_count += diagonal_count_2;
        }

        // Upper-left half
        for col in (0..width - 1).rev() {
            let mut diagonal = String::new();
            let mut curr_row = 0;
            let mut curr_col = col;

            while curr_row < height {
                diagonal.push(lines[curr_row].chars().nth(curr_col).unwrap());
                curr_row += 1;
                curr_col = if curr_col > 0 {
                    curr_col - 1
                } else {
                    break;
                };
            }

            let reverse_diagonal = diagonal.chars().rev().collect::<String>();

            let diagonal_count_1 = count_occurrences_of_substring(&diagonal, pattern, 0);
            let diagonal_count_2 = count_occurrences_of_substring(&reverse_diagonal, pattern, 0);

            total_count += diagonal_count_1;
            total_count += diagonal_count_2;
        }

        println!("Total count: {}", total_count);
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
