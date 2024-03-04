fn main() {
    let mut number = 1;
    let result = fizz_buzz(number);
    println!("From number {number}, the result is {result}");

    number = 3;
    let result = fizz_buzz(number);
    println!("From number {number}, the result is {result}");

    number = 5;
    let result = fizz_buzz(number);
    println!("From number {number}, the result is {result}");

    number = 10;
    let result = fizz_buzz(number);
    println!("From number {number}, the result is {result}");

    number = 15;
    let result = fizz_buzz(number);
    println!("From number {number}, the result is {result}");

    number = 16;
    let result = fizz_buzz(number);
    println!("From number {number}, the result is {result}");
}

fn fizz_buzz(i: u32) -> String {
    if i % 15 == 0 {
        "FizzBuzz".to_string()
    } else if i % 3 == 0 {
        "Fizz".to_string()
    } else if i % 5 == 0 {
        "Buzz".to_string()
    } else {
        i.to_string()
    }
}
