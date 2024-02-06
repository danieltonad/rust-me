use std::mem;

fn main() {
    let o = Object{
        width: 20,
        height: 30
    };

    println!("width: {}, height: {} \n Area: {}", o.width, o.height, area(&o));
    // let sum: i32 = 2;
    // let y: i32 = 0;
    // let mut a : [i32; 10]= [1,2,3,4,5,6,7,8,9,0];

    // tuples_example();

    // let mut v: Vec<i32> = Vec::new();

    // for i in 1..1000{
    //     v.push(i);   
    // }
    // v = re(v);
    // println!("still own v: {} {}", v[0], v[1]);
    
    // borrow1(&v);
    // println!("still own v: {} {}", v[0], v[1]);

    // borrow2(&v);
    // println!("still own v: {} {}", v[0], v[1]);
    
    //     let  y: i32 = 200;
    //     println!("sum = {}, Y = {}", sum, y)
    // }
    
    // println!("sum = {}, Y = {}", sum, y)
    // greet_me("Solarin")
}

fn area(obj: &Object) -> u32 {
    obj.width * obj.height
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

fn string_samples(){
    let literal_string = "Hello Boy!";
    let a = String::from("Hello, ");
    let b = String::from("Daniel");
    let slice = &literal_string[2..5];

    let concat =  a + &b;

    println!("{:#?}", concat);
}

fn re(v: Vec<i32>) -> Vec<i32> {
    println!("{}", v[120] + v[111]);
    v
}

fn borrow1(v: &Vec<i32>){
    println!("{}", (*v)[10] + (*v)[11]);
}

fn borrow2(v: &Vec<i32>){
    println!("{}", v[10] + v[12]);
}

fn count(v: &Vec<i32>, val: i32) -> usize{
    v.into_iter().filter(|&&x| x == val).count()
}

struct Object {
    width: u32,
    height: u32
}