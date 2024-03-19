// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// 6. Tuple Struct
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// 8.
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// 9. Method syntex
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // 1. Basic
    // let mut user1 = User {
    //     active: true,
    //     username: String::from("jaranchai"),
    //     email: String::from("jaranchai.nt@gmail.com"),
    //     sign_in_count: 1,
    // };

    // println!("User1: {:?}", user1);
    // println!("Email: {}", user1.email);

    // 2. Assign
    // user1.email = String::from("anotheremail@example.com");

    // println!("Email: {}", user1.email);

    // 3. Recive from function
    // let user2 = build_user(
    //     String::from("jaranchai.nt@gmail.com"),
    //     String::from("jaranchai"),
    // );
    // println!("User2: {:?}", user2);

    // 5. Creating Instances from Other Instances
    // 5.1 Using Struct Update Syntax
    // let user3 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // println!("User3: {:?}", user3);

    // 5.2 Using Struct Update Syntax by ..user1
    // let user4 = User {
    //     email: String::from("another@example.com"),
    //     ..user1
    // };
    // println!("User4: {:?}", user4);

    // 7. Tuple Struct
    // 7.1 Simple
    // let origin = Point(0, 0, 0);
    // println!("origin 0: {}", origin.0);
    // println!("origin 1: {}", origin.1);
    // println!("origin 2: {}", origin.2);

    // 7.2 Update Tuple Struct signle value
    // let mut black = Color(0, 0, 0);
    // black.0 = 255;

    // println!("black 0: {}", black.0);
    // println!("black 1: {}", black.1);
    // println!("black 2: {}", black.2);

    // 7.3 Update Tuple Struct all
    // black = Color(255, 255, 255);

    // println!("black 0: {}", black.0);
    // println!("black 1: {}", black.1);
    // println!("black 2: {}", black.2);

    // 8. Applying Methods
    // let rect1 = (30, 50);
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(rect1)
    // );

    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(&rect1)
    // );

    // 9. Method syntex
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

// 4. Short hand syntax
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }
