fn main() {
    // Creating a New Vector
    // let v1: Vec<i32> = Vec::new();

    // println!("The value of v1 is: {:?}", v1);

    // let v2 = vec![1, 2, 3];
    // println!("The value of v2 is: {:?}", v2);

    // Updating a Vector
    // let mut v3: Vec<i32> = Vec::new();
    // println!("The value of v3 is: {:?}", v3);
    // v3.push(5);
    // v3.push(6);
    // v3.push(7);
    // v3.push(8);
    // println!("The value of v3 is: {:?}", v3);

    // Reading Elements of Vectors
    let vec = vec![1, 2, 3, 4, 5];

    let third: &i32 = &vec[2];
    println!("The third element is {third}");

    let third: Option<&i32> = vec.get(2);
    let a = &third;
    println!("The third element is ss {a}");
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Slice
    // จาก array
    // let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    // let my_slice: &[i32] = &my_array[1..4];
    // println!("The slice is: {:?}", my_slice);

    // จาก string literal
    // let my_str: &str = "Hello, world!";
    // println!("The my_str is: {:?}", my_str);

    // จาก slice อื่น
    // let my_slice2: &[i32] = my_slice.clone();
    // println!("The my_slice2 is: {:?}", my_slice2);
}
