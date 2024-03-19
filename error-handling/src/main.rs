// 3., 4. Matching errors
use std::fs::File;
// 4. Matching on Different Errors
use std::io::ErrorKind;

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    // 1. Error handling
    // panic!("Something went wrong");

    // 2. Error handling from the standard library
    // let v = vec![1, 2, 3];

    // v[99];

    // 3. Matching errors
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // 4. Matching on Different Errors
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
