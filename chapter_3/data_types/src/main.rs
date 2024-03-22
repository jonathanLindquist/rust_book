fn main() {
    // unsigned int
    let guess: u32 = "43".parse().expect("Not a number!");

    // boolean
    let bool_item: bool = false;

    // remainder (modulo)
    let remainder = 56 % 5;

    // character
    let c = 'c';

    // tuple
    let tuple: (i32, f64, u32) = (500, 6.4, 45);
    let (x,y,z) = tuple;
    println!("The values of the tuple: ({x}, {y}, {z})");
    
    let first_item = tuple.0;
    println!("first item: {first_item}");

    let second_item = tuple.1;
    println!("second item: {second_item}");

    let third_item = tuple.2;
    println!("third item: {third_item}");


    // array
    let a = [1,2,3,4];
    // println!("{a}");
    let second_array: [u32; 5] = [1,2,3,4,5];
    let first_item = second_array[0];
    println!("first item from array: {first_item}");
}
