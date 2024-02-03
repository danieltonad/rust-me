fn main() {
    let sum: i32 = 2;
    let y: i32 = 0;
    
    // {
    //     let  y: i32 = 200;
    //     println!("sum = {}, Y = {}", sum, y)
    // }
    
    // println!("sum = {}, Y = {}", sum, y)
    greet_me("Solarin")
}

fn greet_me(name: &str){
    println!("Hello {}", name);
}
