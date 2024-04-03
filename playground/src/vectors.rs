

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4];

    // add spacific
    numbers.push(123);

    // pop last
    numbers.pop();

    // Loop through vector values
    for i in numbers.iter(){
        println!(":{}:", i)
    }
    
    // Loop & mutate values
    for i in numbers.iter_mut(){
        // println!(":{}:", i)
        *i *= *i;
    }


    println!("{:?}", numbers)
}