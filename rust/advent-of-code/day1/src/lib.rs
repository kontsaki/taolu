use itertools::Itertools;
use std::fs;

pub fn get_expenses(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename).expect("WHERE IS FILE");
    let mut expenses = contents
        .lines()
        .map(|line| line.parse::<i32>().unwrap_or(0))
        .collect::<Vec<_>>();

    expenses.sort();
    expenses
}

//211899
pub fn find_pair(vec: &Vec<i32>, sum: i32) -> Option<(&i32, &i32)> {
    vec.iter().tuple_combinations().find(|&(i, j)| i + j == sum)
}

pub fn find_triplet(vec: &Vec<i32>, sum: i32) -> Option<(&i32, &i32, &i32)> {
    vec.iter()
        .tuple_combinations()
        .find(|&(i, j, k)| i + j + k == sum)
}

pub fn find_pair_declarative(vec0: &Vec<i32>, vec1: &Vec<i32>, sum: i32) -> Option<(i32, i32)> {
    vec0.iter()
        .copied()
        .map(|i| {
            vec1.iter()
                .copied()
                .map(move |j| (i, j))
                .skip_while(|(i, j)| i + j > sum)
        })
        .flatten()
        .find(|(i, j)| i + j == sum)
}

pub fn find_pair_iterative(expenses: &Vec<i32>) -> Option<(&i32, &i32)> {
    for j in expenses.iter().rev() {
        for i in expenses.iter() {
            let sum = i + j;
            if sum == 2020 {
                return Some((i, j));
            } else if sum > 2020 {
                break;
            }
        }
    }
    None
}
