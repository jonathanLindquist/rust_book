fn main() {
    println!("Hello, world!");

    another_function(34);

    // will error
    // let x = (let y = 5);
    let x = {
        let y = 5;
        y + 2
    };

    let x = x + 5;

    println!("x: {x}");

    let x = five();
    println!("five function: {x}");

    let x = plus_one(10);
    println!("plus one function: {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(i: i32) -> i32 {
    i + 1
}

fn another_function(i: u32) {
    println!("Another function!: {i}");
}
