use core::num;
use std::fs;

fn main() {
    let paths = ["inputs/example.txt", "inputs/data.txt"];

    for path in paths {
        let data = fs::read_to_string(path).expect("Should have been able to read the file");

        let reports = data.split('\n');

        let mut num_safe_reports = 0;
        for report in reports {
            num_safe_reports += 1;
            let levels: Vec<i64> = report
                .split(' ')
                .filter_map(|num| num.parse::<i64>().ok())
                .collect();

            let mut movement_started = false;
            let mut last_difference = 0;

            for (idx, elem) in levels[1..].iter().enumerate() {
                println!("{}", elem);
                let curr_difference = elem - levels[idx];

                if curr_difference == 0 {
                    println!("Gotta keep moving");
                    num_safe_reports -= 1;
                    break;
                }

                if curr_difference.abs() > 3 {
                    println!("Too large of a movement");
                    num_safe_reports -= 1;
                    break;
                }

                if !movement_started {
                    last_difference = curr_difference;
                    movement_started = true;
                }

                if (last_difference > 0 && curr_difference < 0)
                    || (last_difference < 0 && curr_difference > 0)
                {
                    println!("Switched signs");
                    num_safe_reports -= 1;
                    break;
                }
            }
        }

        println!("Number of safe reports: {}", num_safe_reports);
    }
}
