fn main() {
    print_hello();

    // return a value
    let result = get_number();
    println!("Number is {}", result);

    // send parmeter and return a value
    let add_result = add(1, 2);
    println!("1 + 2 = {}", add_result);

    // return two values
    let (a, b) = return_two_values(1, 2);
    println!("Set of 1, 2 (multiply 2) => {}, {}", a, b);
}

// fn Name() {}
fn print_hello() {
    println!("Hello, from Rust!");
}

// 
fn get_number() -> i32 {
    let number: i32 = 2;
    
    number
}

// fn Name(argrument: data_type) -> return_type {}
fn add(a: i32, b: i32) -> i32 {
    // return a + b
    a + b
}

// fn Name(argrument: data_type) -> return_type {}
fn return_two_values(a: i32, b: i32) -> (i32, i32) {
    (a * 2, b * 2)
}
