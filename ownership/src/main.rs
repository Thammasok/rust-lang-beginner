fn main() {
    // Scopes
    // let number = 1; // on the stack
    // let mut s = "Hello".to_string(); // on the heap
    // s.push_str(" World!");

    // Move ownership
    // let x = vec!["my name".to_string()];
    // let y = x;
    // // println!("{:?}", x); // value borrowed here after move
    // println!("{:?}", y);

    // Clone
    // let x = vec!["my name".to_string()];
    // let z = x.clone();
    // println!("{:?}", x);
    // println!("{:?}", z);

    // Copy
    let x = 1; // copy only type on the stack such as integer, boolean etc.
    let y = x;
    println!("{:?}", x);
    println!("{:?}", y);
}

// number is dropped, s is dropped
