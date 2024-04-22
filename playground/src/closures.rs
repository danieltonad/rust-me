pub fn run(){
    let multiplier: i32 = 10;
    let sum_mult = |n1: i32, n2: i32| n1 + n2 * multiplier;

    println!("{}", sum_mult(10, 5));
}