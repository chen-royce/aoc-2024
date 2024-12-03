use std::fs;

fn main() {
    let paths = [
        "inputs/example.txt",
        "inputs/data.txt",
        // "inputs/corner-case.txt",
    ];

    for path in paths {
        let mut curr_total = 0;
        let mut count = 0;
        let mut data = fs::read_to_string(path).expect("Should have been able to read the file");

        /*
        Handle do/don't commands by doing the following:
        1. Prepend an explicit "do()" to the start of the data
        2. Split the data into blocks by occurrences of "don't()" and see if blocks
           containing "don't()" also contain any "do()" directives
            a. If yes, then preserve only the instructions after "do()"
            b. If no, drop the block
        4. Rejoin the blocks and run the rest of the algo from part 1
        */

        data.insert_str(0, "do()");
        let mut do_blocks: Vec<String> = data
            .split("don't()")
            .filter_map(|block| {
                // Check if "do()" is present in the block
                if let Some(start_doing_idx) = block.find("do()") {
                    // Slice from "do()" onward and return it
                    Some(block[start_doing_idx..].to_string())
                } else {
                    // Skip the block if "do()" is not found
                    None
                }
            })
            .collect();

        // Join the modified blocks back into a single string
        let data = do_blocks.join("");

        println!("{}", data);

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
