use std::fs::File;
use std::io::{BufRead, BufReader};

advent_of_code::solution!(1);

pub fn part_one(_input: &str) -> Option<u32> {

    let file = File::open("data/inputs/01.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut arr1: [u32; 1000] = [0; 1000];
    let mut arr2: [u32; 1000] = [0; 1000];
    let mut count = 0;
    for line in lines.by_ref() {
        let line = line.unwrap();
        let line: Vec<&str> = line.split_whitespace().collect();
        arr1[count] = line[0].parse::<u32>().unwrap();
        arr2[count] = line[1].parse::<u32>().unwrap();
        count += 1;
    }

    arr1.sort_unstable();
    arr2.sort_unstable();
    let mut sum:u32 = 0;

    for counter in 0..1000 {
        if(arr1[counter] > arr2[counter]) {
            let diff:u32 = arr1[counter] - arr2[counter];
            sum+=diff;
        }
        else {
            let diff:u32 = arr2[counter] - arr1[counter];
            sum+=diff;
        }
    }

    return Some(sum);
}

pub fn part_two(_input: &str) -> Option<u32> {
    let file = File::open("data/inputs/01.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut arr1: [u32; 1000] = [0; 1000];
    let mut arr2: [u32; 1000] = [0; 1000];
    {
        let mut count = 0;
        for line in lines.by_ref() {
            let line = line.unwrap();
            let line: Vec<&str> = line.split_whitespace().collect();
            arr1[count] = line[0].parse::<u32>().unwrap();
            arr2[count] = line[1].parse::<u32>().unwrap();
            count += 1;
        }
    }

    let mut sum = 0;
    for num1 in arr1 {
        let mut count =0;
        for num2 in arr2 {
            if(num1 == num2) {
                count+=1;
            }
        }
        sum += num1 * count;
    }

    return Some(sum);
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
