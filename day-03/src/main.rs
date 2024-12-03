use std::fs;

fn main() {
    let paths = ["inputs/example.txt", "inputs/data.txt"];

    for path in paths {
        let mut curr_total = 0;
        let mut count = 0;
        let data = fs::read_to_string(path).expect("Should have been able to read the file");
        let equations: Vec<&str> = data.split("mul(").collect();

        for eq in &equations[1..] {
            count += 1;
            println!("eq #{}: {}", count, eq);
            let factors: Vec<&str> = eq.split(',').collect();
            if factors.len() < 2 {
                continue;
            }
            let first_factor = factors[0].parse::<u128>().ok();
            let second_factor_string = factors[1].split(')').next();

            let mut second_factor: Option<u128> = None;
            match second_factor_string {
                Some(s) => second_factor = s.parse::<u128>().ok(),
                _ => continue,
            };

            match (first_factor, second_factor) {
                (Some(num1), Some(num2)) => {
                    let product = num1 * num2;
                    println!("{}", product);
                    println!("");
                    curr_total += product;
                }
                _ => continue,
            };
        }

        println!("Total: {}", curr_total);
        println!("--------------");
    }
}
