pub fn run(){
    let age = 24;
    let mut name = "Daniel";

    println!("My name is {0}, I'm {1} years old.", name, age);


    // constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // multiple vars
    let (name, age) = ("Brad", 24);
    println!("{} is {}", name, age);

    // shorthand If
    let is_of_age = if age >= 21 {true} else {false};
}