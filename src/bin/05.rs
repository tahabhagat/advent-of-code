use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {


    let mut rules:HashMap<u32, Vec<u32>> = HashMap::new();
    let mut count_middle_pages:u32 = 0;
    let mut rules_complete = false;

    for line in input.lines() {
        if line.is_empty() { //reached end of rules
            rules_complete = true;
            continue;
        }
        if !rules_complete {
            let nums: Vec<&str> = line.split("|").collect();

            match rules.get_mut(&nums[0].parse::<u32>().unwrap()) {
                None => {
                    rules.insert(nums[0].parse::<u32>().unwrap(), vec![nums[1].parse::<u32>().unwrap()]);
                }
                Some(existing_rules) => {
                    existing_rules.push(nums[1].parse::<u32>().unwrap());
                }
            }
        } else { //logic to get each line

            let book: Vec<u32> = line.split(",").map(
                |n| n.trim().parse::<u32>().unwrap()
            ).collect();

            let mut book_is_valid = true;
            let mut prev_pages: HashSet<u32> = HashSet::new();
            'pages: for page in &book {
                if rules.contains_key(&page) { //rule is present for given page

                    let page_rule = rules.get(&page).unwrap();
                    for rule in page_rule {
                        if prev_pages.contains(&rule) { //not a  valid book, as rule was violated
                            book_is_valid = false;
                            break 'pages;
                        }
                    }
                    //means no rule was broken for this page, can add page to prev_pages and check next page
                    prev_pages.insert(*page);
                }
            }
            if book_is_valid {
                let middle_page = book.get(book.len() / 2).unwrap();
                count_middle_pages+=middle_page;
            }
        }
    }


    Some(count_middle_pages)
}

pub fn part_two(input: &str) -> Option<u32> {

    let mut rules:HashMap<u32, Vec<u32>> = HashMap::new();
    let mut count_middle_pages:u32 = 0;
    let mut rules_complete = false;

    for line in input.lines() {
        if line.is_empty() { //reached end of rules
            rules_complete = true;
            continue;
        }
        if !rules_complete {
            let nums: Vec<&str> = line.split("|").collect();

            match rules.get_mut(&nums[0].parse::<u32>().unwrap()) {
                None => {
                    rules.insert(nums[0].parse::<u32>().unwrap(), vec![nums[1].parse::<u32>().unwrap()]);
                }
                Some(existing_rules) => {
                    existing_rules.push(nums[1].parse::<u32>().unwrap());
                }
            }
        } else { //logic to get each line

            let mut book: Vec<u32> = line.split(",").map(
                |n| n.trim().parse::<u32>().unwrap()
            ).collect();

            let mut book_is_valid = true;
            let mut prev_pages: HashSet<u32> = HashSet::new();
            'pages: for page in &book {
                if rules.contains_key(&page) { //rule is present for given page

                    let page_rule = rules.get(&page).unwrap();
                    for rule in page_rule {
                        if prev_pages.contains(&rule) { //not a  valid book, as rule was violated
                            book_is_valid = false;
                            break 'pages;
                        }
                    }
                    //means no rule was broken for this page, can add page to prev_pages and check next page
                    prev_pages.insert(*page);
                }
            }
            if !book_is_valid {
                book.sort_by(
                    |a,b| {
                        if rules.contains_key(&a) {
                            let a_rule = rules.get(&a).unwrap();
                            if a_rule.contains(b) { //means b should be after a
                                return Ordering::Less;
                            }
                        }
                        if rules.contains_key(&b) {
                            let b_rule = rules.get(&b).unwrap();
                            if b_rule.contains(a) { // means b should be before a
                                return Ordering::Greater;
                            }
                        }
                        Ordering::Equal
                    }
                );
                let middle_page = book.get(book.len() / 2).unwrap();
                count_middle_pages+=middle_page;
            }
        }



    }


    Some(count_middle_pages)
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
