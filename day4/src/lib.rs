pub fn part_a(input: &str) -> i64 {
    let mut sum = 0;

    for line in input.trim().split("\n") {
        let range_strs = line.split(",");

        let first_range_str = *range_strs.clone().collect::<Vec<&str>>().first().unwrap();

        let first_nums: Vec<&str> = first_range_str.split("-").collect();

        let first_lower_bound = first_nums[0].parse::<i32>().unwrap();
        let first_upper_bound = first_nums[1].parse::<i32>().unwrap();

        let last_range_str = *range_strs.collect::<Vec<&str>>().last().unwrap();
        let last_nums: Vec<&str> = last_range_str.split("-").collect();

        let last_lower_bound = last_nums[0].parse::<i32>().unwrap();
        let last_upper_bound = last_nums[1].parse::<i32>().unwrap();

        if (first_lower_bound >= last_lower_bound && first_upper_bound <= last_upper_bound)
            || (first_lower_bound <= last_lower_bound && first_upper_bound >= last_upper_bound)
        {
            println!(
                "\n\nline: {}\nfirst range: {}-{}, last range: {}-{}",
                line, first_lower_bound, first_upper_bound, last_lower_bound, last_upper_bound
            );
            sum += 1
        }
    }

    sum
}

pub fn part_b(input: &str) -> i64 {
    let mut sum = 0;

    for line in input.trim().split("\n") {
        let range_strs = line.split(",");

        let first_range_str = *range_strs.clone().collect::<Vec<&str>>().first().unwrap();

        let first_nums: Vec<&str> = first_range_str.split("-").collect();

        let first_lower_bound = first_nums[0].parse::<i32>().unwrap();
        let first_upper_bound = first_nums[1].parse::<i32>().unwrap();

        let last_range_str = *range_strs.collect::<Vec<&str>>().last().unwrap();
        let last_nums: Vec<&str> = last_range_str.split("-").collect();

        let last_lower_bound = last_nums[0].parse::<i32>().unwrap();
        let last_upper_bound = last_nums[1].parse::<i32>().unwrap();

        if (first_lower_bound >= last_lower_bound && first_upper_bound <= last_upper_bound)
            || (first_lower_bound <= last_lower_bound && first_upper_bound >= last_upper_bound)
            || (first_upper_bound >= last_lower_bound && first_lower_bound <= last_lower_bound)
            || (first_upper_bound <= last_lower_bound && first_lower_bound >= last_lower_bound)
            || (last_upper_bound >= first_lower_bound && last_lower_bound <= first_lower_bound)
            || (last_upper_bound <= first_lower_bound && last_lower_bound >= first_lower_bound)
        {
            println!(
                "\n\nline: {}\nfirst range: {}-{}, last range: {}-{}",
                line, first_lower_bound, first_upper_bound, last_lower_bound, last_upper_bound
            );
            sum += 1
        }
    }

    sum
}

#[cfg(test)]
#[cfg(test)]
mod test {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 0)
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 0)
    }
}
