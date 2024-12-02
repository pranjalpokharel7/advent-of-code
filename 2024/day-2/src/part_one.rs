pub fn check_level_safety(levels: &Vec<i32>) -> bool {
    let mut previous_diff = 0;
    let mut level_safe = true;
    for i in 0..levels.len() - 1 {
        let diff: i32 = levels[i + 1] - levels[i];
        if !(diff.abs() >= 1
            && diff.abs() <= 3
            && ((previous_diff >= 0 && diff > 0) || (previous_diff <= 0 && diff < 0)))
        {
            level_safe = false;
            break;
        }
        previous_diff = diff;
    }

    level_safe
}

#[allow(dead_code)]
pub fn part_one(lines: Vec<&str>) -> u32 {
    let mut safe_count: u32 = 0;

    for line in lines {
        let levels: Vec<i32> = line
            .split(' ')
            .map(|level| level.parse().expect("Error - Not a valid number: {level}"))
            .collect();

        if check_level_safety(&levels) {
            safe_count += 1;
        }
    }

    safe_count
}
