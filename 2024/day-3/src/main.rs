use regex::Regex;
use std::fs;

fn parse_and_multiply(re: &Regex, content: &str) -> u32 {
    re.captures_iter(content)
        .map(|mul| {
            let (_, [a, b]) = mul.extract();
            let au32 = a.parse::<u32>().unwrap();
            let bu32 = b.parse::<u32>().unwrap();
            (au32, bu32)
        })
        .fold(0, |acc, (a, b)| acc + (a * b))
}

fn main() {
    let file_path = "data/input.txt";
    let contents = fs::read_to_string(file_path).expect("Unable to read file");

    let dont_split: Vec<&str> = contents.split("don't()").collect();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut product: u32 = 0;

    for i in 0..dont_split.len() {
        let do_split: Vec<&str> = dont_split[i].split("do()").collect();

        if i == 0 {
            product += parse_and_multiply(&re, dont_split[i]);
        } else if do_split.len() > 1 {
            for j in 1..do_split.len() {
                product += parse_and_multiply(&re, do_split[j]);
            }
        }
    }
    println!("{:?}", product);
}
