#![warn(clippy::pedantic)]
use std::io::stdin;
use std::collections::{HashMap, HashSet};

fn get_int() -> Option<i64> {
    println!("Enter an integer: ");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Failed to read line");
    if buffer.trim().is_empty() {
        return None;
    }
    match buffer.trim().parse::<i64>() {
        Ok(int) => Some(int),
        Err(value) => {
            eprintln!("Could not read data: {value}");
            get_int()
        }
    }
}

fn get_average(integers: &[i64]) -> Option<i64> {
    if integers.is_empty() {
        return None
    }

    #[allow(clippy::cast_possible_wrap)]
    Some(integers.iter().sum::<i64>() / integers.len() as i64)
}

fn get_median(integers: &[i64]) -> Option<i64> {
    if integers.is_empty() {
        return None
    }

    let mut temp_ints = integers.to_owned();
    temp_ints.sort_unstable();
    Some(temp_ints[temp_ints.len() / 2])
}

fn get_mode(integers: &[i64]) -> Option<i64> {

    let set: HashSet<i64> = integers.iter().copied().collect();
    // set removes dups, no mode if only unqiue nums in integers
    if set.len() == integers.len() {
        return None;
    }


    let mut value_occurances = HashMap::new();
    for i in integers {
        let occurances = value_occurances.entry(i).or_insert(0);
        *occurances += 1;
    }

    let mut mode = integers[0];
    for (number, occurance_count) in &value_occurances {
        if *occurance_count > mode {
            mode = **number;
        }
    }
    Some(mode)
}

fn main() {
    println!("Getting integers ('enter' to quit). . .");

    let mut integers: Vec<i64> = Vec::new();
    while let Some(int) = get_int() {
        integers.push(int);
    }

    println!("------------------------------------------");
    println!("Entered: {integers:?}");
    println!("Average {}", get_average(&integers).map_or("None".to_string(), |i|i.to_string()));
    println!("Median {}", get_median(&integers).map_or("None".to_string(), |i|i.to_string()));
    println!("Mode {}", get_mode(&integers).map_or("None".to_string(), |i|i.to_string()));

}
