use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn main() {
    let paths = ["inputs/example.txt", "inputs/data.txt"];

    for path in paths {
        let data = fs::read_to_string(path).expect("Should have been able to read the file");

        let list_one: Vec<i64> = get_nth_elements_from_inputs(&data, 0);
        let list_two: Vec<i64> = get_nth_elements_from_inputs(&data, 1);

        let sum: i64 = part_1(&list_one, &list_two);
        let similarity_score: i64 = part_2(&list_one, &list_two);

        println!("{} part 1: {}", path, sum);
        println!("{} part 2: {}", path, similarity_score);
        println!("");
    }
}

fn part_1(list_one: &Vec<i64>, list_two: &Vec<i64>) -> i64 {
    list_one
        .into_iter()
        .zip(list_two.into_iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn part_2(list_one: &Vec<i64>, list_two: &Vec<i64>) -> i64 {
    let freqs = vec_to_freq_map(list_two);
    list_one
        .iter()
        .filter_map(|&num| freqs.get(&num).map(|&freq| num * freq as i64))
        .sum()
}

fn vec_to_freq_map(vec: &Vec<i64>) -> HashMap<i64, usize> {
    let mut freq_map = HashMap::new();
    for num in vec {
        *freq_map.entry(*num).or_insert(0) += 1;
    }
    freq_map
}

fn get_nth_elements_from_inputs(data: &str, n: usize) -> Vec<i64> {
    data.split('\n')
        .filter_map(|line| line.split("   ").nth(n)?.parse::<i64>().ok())
        .into_iter()
        .sorted()
        .collect()
}
