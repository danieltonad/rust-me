
pub fn run() {
    // positional args
    println!("{0} is a fine {1}, \n{0} ain't no pussy", "Daniel", "boy");
    //   Named Args
    println!("{name} is a fine {gender}, \n{name} ain't no pussy",name="Daniel", gender="Male");
    // placeholder traits + positional args
    println!("Binary: {0:b} Hex: {0:x} Octal: {0:o} ", 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));
  
}