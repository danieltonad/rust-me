use std::io::{self, Write};

pub fn run(){

    print!("Enter Text: ");
    io::stdout().flush().unwrap();

    let mut text = String::new();
    io::stdin().read_line(&mut text)
        .expect("Unable to read text");

    let chars: Vec<char> = text.chars().filter(|&c| c != '\r' && c != '\n').collect();

    for i in 1..=chars.len() {
        println!("{}", &text[0..i]);
    }

}