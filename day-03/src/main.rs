use std::fs;

fn main() {
    let paths = [
        "inputs/example.txt",
        "inputs/data.txt",
        "inputs/corner-case.txt",
    ];

    for path in paths {
        let mut curr_total = 0;
        let mut count = 0;
        let data = fs::read_to_string(path).expect("Should have been able to read the file");
        let equations: Vec<&str> = data.split("mul(").collect();

        for eq in &equations[1..] {
            let factors: Vec<&str> = eq.split(',').collect();
            if factors.len() < 2 {
                continue;
            }
            let first_factor = factors[0].parse::<u128>().ok();

            let closing_paren_index = factors[1].find(')');
            let second_factor_string = match closing_paren_index {
                Some(index) => &factors[1][..index],
                None => continue,
            };

            let second_factor = second_factor_string.parse::<u128>().ok();

            match (first_factor, second_factor) {
                (Some(num1), Some(num2)) => {
                    count += 1;
                    println!("eq #{}: {}", count, eq);
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
