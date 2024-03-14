fn main() {
    // Scopes
    // let number = 1; // on the stack
    let mut s = "Hello".to_string(); // on the heap
    s.push_str(" World!");
    println!("s: {}", s);
    // let s2 = s;
    // println!("s2: {}", s2);

    // Copy
    // let x = 1; // copy only type on the stack such as integer, boolean etc.
    // let y = x;
    // println!("x: {}", x);
    // println!("y: {}", y);

    // Copy to Function
    // let x = 1;
    // let y = x;
    // make_copy(x);
    // println!("x: {}", x);
    // println!("y: {}", y);

    // Ownership
    // Move ownership
    // let x = String::from("Nutshell");
    // let y = x;
    // println!("x: {}", x); // Error: value borrowed here after move
    // println!("y: {}", y);

    // Clone
    // let x = String::from("Nutshell");
    // let z = x.clone();
    // println!("x: {}", x);
    // println!("y: {}", z);

    // Move to Function (Give ownership to the function)
    // let msg = String::from("Hello"); // create a variable with a string "Hello"
    // takes_ownership(msg); // Give ownership to the function
    // println!("{}", msg); // Error: value borrowed here after move

    // Give ownership to the function
    // let msg: String = give_ownership();
    // println!("msg: {}", msg);

    // Reference and borrow
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is {}.", s1, len);

    // add mut if you want to change the string
    // let mut s = String::from("Hello");
    // change_string(&mut s); // &s is a reference to s
    // println!("s: {}", s);

    // Slices
    // let s = String::from("hello world");

    // let hello = &s[0..5];
    // let world = &s[6..11];
    // println!("{} {}", hello, world);
}

// number is dropped, s is dropped

// Move to Function
// fn takes_ownership(some_string: String) {
//     println!("takes_ownership: {}", some_string);
// }

// fn give_ownership() -> String {
//     let some_string = String::from("Given");
//     some_string
// }

// Copy to Function
// fn make_copy(one: i32) {
//     println!("make_copy: {}", one);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn change_string(some_string: &mut String) { // add mut if you want to change the string
//     some_string.push_str(", world"); // `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
// }
