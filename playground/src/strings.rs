pub fn run(){

    // char
    // let a1  ='a';
    // let emoji  ='\u{1f600}';

    let mut name = String::from("value " );
    name.push('-');
    name.push_str(" new");

    // println!("Capacity: {}", name.capacity());
    
    // println!("Is Empty: {}", name.is_empty());

    // println!("Contains: 'w' {}", name.contains('w'));

    // println!("Replace:  {}", name.replace("new", "old"));
    // for word in name.split_whitespace(){
    //     println!("{}", word);
    // }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push_str(" - string");

    assert_eq!(10, s.capacity());

    println!("{}", s)

}