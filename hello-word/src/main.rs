use std::mem;

fn main() {
    let sum: i32 = 2;
    let y: i32 = 0;
    let mut a : [i32; 10]= [1,2,3,4,5,6,7,8,9,0];

    tuples_example();

    array_samples();
    
    //     let  y: i32 = 200;
    //     println!("sum = {}, Y = {}", sum, y)
    // }
    
    // println!("sum = {}, Y = {}", sum, y)
    // greet_me("Solarin")
}

// func args
fn greet_me(name: &str){
    println!("Hello {}", name);
}

fn tuples_example(){
    let t = (42, 6.12, 'h', 3, 4, 4, 4, 5,8 , 88, 69, 23, 49, 47, 903, 933, 893, 28383, 29292, 22);
    // (a, b, c) = t;
    // println!("{}, {}, {}", t.0, t.1, t.2)
    // println!("{:#?}", t);
}

fn array_samples(){
    let arr: [i32; 5] = [1,2,3,4,5];

    let part = & arr[2..4];

    println!("Part size: {} \n Mem: {}", part.len(), mem::size_of_val(&part));
}

f