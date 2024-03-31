fn main() {
    let s1 = String::from("Hello!");

    let len = calculate_length(&s1);

    // change(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("World!");

    change(&mut s);

    {
        let mut_x = &mut s;
        
        // more that 1 mutable reference to the same variable will fail to compile
        // let mut_y = &mut s;
    }

    let mut_y = &mut s; // this is fine since mut_x is no longer in scope

    ////////////////////////////

    let mut s50 = String::from("hello");

    let r1 = &s50; // no problem
    let r2 = &s50; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s50; // no problem
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

//   fn change(some_string: &String) {
//     some_string.push_str(", world");
// } // this function will fail

fn change(some_string: &mut String) {
    some_string.push_str(", world");
} // this works