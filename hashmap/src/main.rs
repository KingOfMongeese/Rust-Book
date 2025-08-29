#![warn(clippy::pedantic)]

use std::collections::HashMap;

fn main() {
    let mut dict: HashMap<&str, i32> = HashMap::new();

    dict.insert("banana", 3);
    dict.insert("apple", 0);
    dict.insert("mango", 7);

    for (fruit, count) in &dict {
        println!("{fruit}: {count}");
    }

    if let Some(banana_cnt) = dict.get("banana") {
        println!("Found banana {banana_cnt}");
    }

    if dict.contains_key("orange") {
        println!("found orange!");
    } else {
        println!("No orange");
    }

    for _ in 1..=5 {
        add_grape(&mut dict);
    }

    println!("{dict:?}");

}

fn add_grape(data: &mut HashMap<&str, i32>) {

    data.entry("grape").and_modify(|cnt| *cnt += 1).or_insert(1);
}
