advent_of_code::solution!(4);

use std::ops::Index;
use array2d::Array2D;
use regex::Regex;
use tinyjson::JsonValue::Array;
use itertools::Itertools;
pub fn part_one(input: &str) -> Option<u32> {

    let re = Regex::new(r"XMAS").unwrap();
    let re_rev = Regex::new(r"SAMX").unwrap();


    let mut arr: [[char; 140] ;140] = [[' '; 140]; 140]; // init default value array

    let mut i: usize = 0;
    for line in input.lines() {
        let mut j: usize = 0;
        for char in line.chars() {
            arr[i][j] = char;
            j+=1;
        }
        i += 1;
    }

    let mut xmas_count:u32 = 0;

    for row in &arr {
        let row_str = String::from_iter(row);
        for _matches in re.find_iter(&row_str) {
            xmas_count+=1;
        }
        for _matches in re_rev.find_iter(&row_str) {
            xmas_count+=1;
        }
    }

    for cols in 0..140 {
        let mut col:[char;140] = [' ';140];
        for rows in 0..140 {
            col[rows] = arr[rows][cols];
        }
        let col_str = String::from_iter(col);
        for _matches in re.find_iter(&col_str) {
            xmas_count+=1;
        }
        for _matches in re_rev.find_iter(&col_str) {
            xmas_count+=1;
        }
    }

    for start_col in 0..140 {
        let mut i = 0;
        let mut j = start_col;
        let mut diag:Vec<char> = Vec::new();
        while i < 140 && j < 140 {
            diag.push(arr[i][j]);
            i += 1;
            j += 1;
        }
        let diag_str = String::from_iter(diag);
        for _matches in re.find_iter(&diag_str) {
            xmas_count+=1;
        }
        for _matches in re_rev.find_iter(&diag_str) {
            xmas_count+=1;
        }
    }

    for start_row in 1..140 {
        let mut i = start_row;
        let mut j = 0;
        let mut diag:Vec<char> = Vec::new();
        while i < 140 && j < 140 {
            diag.push(arr[i][j]);
            i += 1;
            j += 1;
        }
        let diag_str = String::from_iter(diag);
        for _matches in re.find_iter(&diag_str) {
            xmas_count+=1;
        }
        for _matches in re_rev.find_iter(&diag_str) {
            xmas_count+=1;
        }
    }

    for start_row in 0..140 {
        let mut i = start_row;
        let mut j = 0;
        let mut diag:Vec<char> = Vec::new();
        while i >= 0 && j < 140 {
            diag.push(arr[i][j]);
            if i == 0 { break; } // Prevent underflow
            i -= 1;
            j += 1;
        }
        let diag_str = String::from_iter(diag);
        for _matches in re.find_iter(&diag_str) {
            xmas_count+=1;
        }
        for _matches in re_rev.find_iter(&diag_str) {
            xmas_count+=1;
        }
    }

    for start_col in 1..140 {
        let mut i = 140 - 1;
        let mut j = start_col;
        let mut diag:Vec<char> = Vec::new();
        while j < 140 {
            diag.push(arr[i][j]);
            if i == 0 { break; } // Prevent underflow
            i -= 1;
            j += 1;
        }
        let diag_str = String::from_iter(diag);
        for _matches in re.find_iter(&diag_str) {
            xmas_count+=1;
        }
        for _matches in re_rev.find_iter(&diag_str) {
            xmas_count+=1;
        }
    }


    Some(xmas_count)
}

pub fn part_two(input: &str) -> Option<u32> {

    let mut arr: [[char; 140] ;140] = [[' '; 140]; 140]; // init default value array

    let mut i: usize = 0;
    for line in input.lines() {
        let mut j: usize = 0;
        for char in line.chars() {
            arr[i][j] = char;
            j+=1;
        }
        i += 1;
    }

    let mut xmas_count:u32 = 0;

    for i in 1..139 {
        for j in 1..139 {
            let c = arr[i][j];
            if c == 'A' { // centre of cross
                if arr[i-1][j-1] == 'M' && arr[i-1][j+1] == 'M' && arr[i+1][j-1] == 'S' && arr[i+1][j+1] == 'S' {
                    xmas_count+=1;
                }
                if arr[i-1][j-1] == 'M' && arr[i-1][j+1] == 'S' && arr[i+1][j-1] == 'M' && arr[i+1][j+1] == 'S' {
                    xmas_count+=1;
                }
                if arr[i-1][j-1] == 'S' && arr[i-1][j+1] == 'S' && arr[i+1][j-1] == 'M' && arr[i+1][j+1] == 'M' {
                    xmas_count+=1;
                }
                if arr[i-1][j-1] == 'S' && arr[i-1][j+1] == 'M' && arr[i+1][j-1] == 'S' && arr[i+1][j+1] == 'M' {
                    xmas_count+=1;
                }

            }

        }
    }

    Some(xmas_count)
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
