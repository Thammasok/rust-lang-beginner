fn main() {
    let mut x = -5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    const PI: f64 = 3.14159265;

    println!("The value of PI is: {PI}");

    // Data Type
    let number: i32 = 5;
    println!("The value of number is: {number}");

    let radiaus: f64 = 3.0;
    println!("The value of radiaus is: {radiaus}");

    let boolean: bool = true;
    println!("The value of boolean is: {boolean}");

    let character: char = 'a';
    println!("The value of character is: {character}");

    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tuple is: {tuple:?}");

    // Array and Slice
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of array is: {array:?}");

    let slice: &[i32] = &array[1..3];
    println!("The value of slice is: {slice:?}");

    let text = String::from("Hello");
    println!("The value of text is: {text}");
}
