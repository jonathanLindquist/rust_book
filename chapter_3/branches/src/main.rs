fn main() {
    let condition = true;

    let result = if condition { 5 } else { 6 };

    // compile error
    // let result = if condition { 5 } else { "string" };

    println!("The result: {result}");

    ////////////////////////

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the counter is: {result}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    ////////////////////////

    let a = [10,20,30,40,50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..3).rev() {
        println!("{number}!");
    }
}
