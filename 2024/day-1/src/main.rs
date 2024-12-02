use std::{collections::HashMap, fs};

fn part_one_calculate_distance(numbers_col_first: &Vec<i32>, numbers_col_second: &Vec<i32>) -> i32 {
    let mut distance: i32 = 0;

    for i in 0..numbers_col_first.len() {
        let diff = numbers_col_first[i] - numbers_col_second[i];
        distance = if diff > 0 {
            distance + diff
        } else {
            distance - diff
        };
    }

    distance
}

fn part_two_calculate_similarity(numbers_col_first: &Vec<i32>, numbers_col_second: &Vec<i32>) -> i32 {
    let mut similarity: i32 = 0;

    let mut element_count: HashMap<i32, i32> = HashMap::new();

    for n in numbers_col_second {
        let count = match element_count.get(n) {
            None => 1,
            Some(x) => x + 1,
        };
        element_count.insert(*n, count);
    }

    for n in numbers_col_first {
        similarity += match element_count.get(n) {
            None => 0,
            Some(x) => (*x) * n,
        }
    }

    similarity
}

fn main() {
    let file_path = "data/input.txt";
    let contents = fs::read_to_string(file_path).expect("Unable to read file");

    let lines: Vec<&str> = contents.split('\n').collect();
    let mut numbers_col_first: Vec<i32> = Vec::with_capacity(lines.len());
    let mut numbers_col_second: Vec<i32> = Vec::with_capacity(lines.len());

    for l in lines {
        let line_split: Vec<&str> = l.split(' ').collect();
        let a = line_split[0].parse().expect("Not a number");
        let b = line_split[3].parse().expect("Not a number");
        numbers_col_first.push(a);
        numbers_col_second.push(b);
    }

    numbers_col_first.sort();
    numbers_col_second.sort();

    let distance = part_one_calculate_distance(&numbers_col_first, &numbers_col_second);
    println!("{}", distance);

    let similarity = part_two_calculate_similarity(&numbers_col_first, &numbers_col_second);
    println!("{}", similarity);
}
