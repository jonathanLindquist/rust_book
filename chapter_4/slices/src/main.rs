fn main() {
    let mut s = String::from("Hello World!");

    let _hello = &s[0..5];
    let hello = &s[..5]; // same as above

    let _world = &s[6..11];
    let _world_2 = &s[6..s.len()];
    let world = &s[6..];

    println!("{} {}", hello, world);

    let sliced_word = first_word(&s);
    println!("{}", sliced_word);

    s.clear();

    // immutable borrow, references a value that's changed, won't compile
    // println!("{} {}", hello, world);

    ////////////////////////////////////

    // changing first_word param from &String to &str

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    //////////////////////////////////////

    let a = [1,2,3,4,5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2,3]);
}

// initial implementation
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}