fn main() {
    // Scopes
    // let number = 1; // on the stack
    // let mut s = "Hello".to_string(); // on the heap
    // s.push_str(" World!");

    // Move ownership
    // let x = vec!["my name".to_string()];
    // let y = x;
    // // println!("{:?}", x); // Error: value borrowed here after move
    // println!("{:?}", y);

    // Clone
    // let x = vec!["my name".to_string()];
    // let z = x.clone();
    // println!("{:?}", x);
    // println!("{:?}", z);

    // Copy
    // let x = 1; // copy only type on the stack such as integer, boolean etc.
    // let y = x;
    // println!("{:?}", x);
    // println!("{:?}", y);

    // Move to Function (Give ownership to the function)
    // let msg = String::from("Hello"); // create a variable with a string "Hello"
    // takes_ownership(msg); // Give ownership to the function
    //                       // println!("{}", msg); // Error: value borrowed here after move

    // Copy to Function
    // let x = 1;
    // let y = x;
    // make_copy(x);
    // println!("{}", x);

    // Give ownership to the function
    // let msg: String = give_ownership();
    // println!("{}", msg);

    // Reference and borrow
    // add mut if you want to change the string
    let mut s = String::from("Hello");
    change_string(&mut s); // &s is a reference to s
    println!("{}", s);
}

// number is dropped, s is dropped

// Move to Function
// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// Copy to Function
// fn make_copy(one: i32) {
//     println!("{}", one);
// }

// fn give_ownership() -> String {
//     let some_string = String::from("Given");
//     some_string
// }

fn change_string(some_string: &mut String) { // add mut if you want to change the string
    some_string.push_str(", world"); // `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}
