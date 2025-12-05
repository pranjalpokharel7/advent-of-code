struct IDRange {
    start: u64,
    end: u64,
}

fn is_invalid(id: u64) -> bool {
    let mut s = id.to_string();
    let len = s.len();

    // if it's not even length, it can not be repeating twice
    if (len & 1) != 0 {
        return false;
    }

    let end = s.split_off(len / 2);
    s == end
}

impl IDRange {
    fn new(range_str: &str) -> Self {
        let mut parts = range_str.split("-");
        let start = parts.next().unwrap();
        let end = parts.next().unwrap();

        let start_u64 = start.parse().expect(&format!("{} is not a valid number", start));
        let end_u64 = end.parse().expect(&format!("{} is not a valid number", end));

        IDRange { start: start_u64, end: end_u64 }
    }

    fn sum_invalid_ids(&self) -> u64 {
        let mut sum = 0;
        for id in self.start..=self.end {
            if is_invalid(id) {
                sum += id;
            }
        }
        sum
    }
}

fn calculate_invalid_id_sum(ids: &Vec<IDRange>) -> u64 {
    let mut sum = 0;
    for id in ids {
        sum += id.sum_invalid_ids();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_from_file;

    #[test]
    fn test_sample() {
        let contents = read_from_file("data/day_two/sample.txt");
        let ranges: Vec<IDRange> = contents
            .split(",")
            .map(|s| IDRange::new(s))
            .collect();
        let sum = calculate_invalid_id_sum(&ranges);
        assert_eq!(sum, 1227775554);
    }

    #[test]
    fn test_input() {
        let contents = read_from_file("data/day_two/input.txt");
        let ranges: Vec<IDRange> = contents
            .split(",")
            .map(|s| IDRange::new(s))
            .collect();
        let sum = calculate_invalid_id_sum(&ranges);
        assert_eq!(sum, 16793817782);
    }
}
