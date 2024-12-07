use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum: u32 = 0;
    for cap in re.captures_iter(input) {
        let num1 = cap[1].parse::<u32>().unwrap();
        let num2 = cap[2].parse::<u32>().unwrap();
        sum += num1 * num2;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut multiply: bool = true;
    let mut sum: u32 = 0;
    for cap in re.captures_iter(input) {
        if cap[0].eq("do()") {
            multiply = true;
        } else if cap[0].eq("don't()") {
            multiply = false;
        } else if multiply {
            let num1 = cap[1].parse::<u32>().unwrap();
            let num2 = cap[2].parse::<u32>().unwrap();
            sum += num1 * num2;
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
