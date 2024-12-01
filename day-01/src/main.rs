use itertools::Itertools;
use std::fs;

fn main() {
    let paths = ["inputs/example.txt", "inputs/data.txt"];

    for path in paths {
        let data = fs::read_to_string(path).expect("Should have been able to read the file");

        let list_one: Vec<i64> = get_nth_elements_from_inputs(&data, 0);
        let list_two: Vec<i64> = get_nth_elements_from_inputs(&data, 1);

        let sum: i64 = part_1(&list_one, &list_two);

        println!("{}", sum);
    }
}

fn part_1(list_one: &Vec<i64>, list_two: &Vec<i64>) -> i64 {
    list_one
        .into_iter()
        .zip(list_two.into_iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}
fn get_nth_elements_from_inputs(data: &String, n: usize) -> Vec<i64> {
    data.split('\n')
        .filter_map(|line| line.split("   ").nth(n)?.parse::<i64>().ok())
        .into_iter()
        .sorted()
        .collect()
}
