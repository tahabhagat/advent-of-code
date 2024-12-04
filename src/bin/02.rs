use std::fs::File;
use std::io::{BufRead, BufReader};

advent_of_code::solution!(2);

pub fn part_one(_input: &str) -> Option<u32> {
    let file = File::open("data/inputs/02.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut num_safe_reports = 0;

    for line in lines.by_ref() {

        let line = line.unwrap();
        let num_list: Vec<&str> = line.split_whitespace().collect();

        // println!("Processing line: {line}");

        if is_report_safe(&line, &num_list) {
            // println!("Report {line} safe! ");
            num_safe_reports += 1;
        } else {
            // println!("Report {line} not safe! ");
        }

    }

    Some(num_safe_reports)
}

fn is_report_safe(_line: &String, num_list: &Vec<&str>) -> bool {
    let mut report_dir: i8 = 0;
    for i in 0..num_list.len() - 1 {
        let diff: i32 = num_list[i].parse::<i32>().unwrap() - num_list[i + 1].parse::<i32>().unwrap();
        if diff == 0 || diff > 3 || diff < -3 {
            // println!("Diff {diff} too high! skipping at elem {i}");
            return false;
        }
        if report_dir == 0 { // first 2 elements
            if diff > 0 {
                report_dir = 1;
            } else {
                report_dir = -1;
            }
        } else if report_dir == 1 && diff < 0 {
            // println!("Diff {diff} wrong way! skipping at elem {i}");
            return false;
        } else if report_dir == -1 && diff > 0 {
            // println!("Diff {diff} wrong way! skipping at elem {i}");
            return false;
        }
    }
    true
}

pub fn part_two(_input: &str) -> Option<u32> {

    let file = File::open("data/inputs/02.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut num_safe_reports = 0;

    'lines: for line in lines.by_ref() {
        let line = line.unwrap();
        let num_list: Vec<&str> = line.split_whitespace().collect();
        if is_report_safe(&line, &num_list) {
            num_safe_reports += 1;
            continue;
        } else {
            for i in 0..line.split_whitespace().count() {

                let mut num_list_clone = num_list.clone();
                num_list_clone.remove(i);
                if is_report_safe(&line, &num_list_clone) {
                    num_safe_reports +=1;
                    continue 'lines;
                }
            }
        }
    }

    Some(num_safe_reports)
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
