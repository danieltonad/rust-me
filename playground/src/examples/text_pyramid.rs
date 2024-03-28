use std::io::{self, Write};

pub fn run(){

    print!("Enter Text: ");
    io::stdout().flush().unwrap();

    let mut text = String::new();
    io::stdin().read_line(&mut text)
        .expect("Unable to read text");

    println!("Text: {}", text)
}