use std::io::stdin;

fn main() {
    let mut in_buffer = String::new();
    println!("Enter String to translate\n>");
    stdin().read_line(&mut in_buffer).expect("Failed to read line");

    let mut translated = String::new();
    for word in in_buffer.trim().split_ascii_whitespace() {

        if word.starts_with(['a', 'e', 'i', 'o', 'u']) {
            translated = format!("{translated} {word}-hay ")
        } else {
            let new_str = format!("{}-{}ay ", word.chars().skip(1).collect::<String>(), word.chars().next().unwrap());
            translated += &new_str;
        }
    }

    println!("{translated}");
}
