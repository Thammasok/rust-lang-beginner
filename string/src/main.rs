fn main() {
    let mut text: &str = "Hello, world!";
    if text == "Hello, world!" {
        text = "Hello, Rustsssss";
        println!("Say {}", text);
    }

    let name = String::from("Rust");
    if name == "Rust" {
        println!("Say {}", name.to_uppercase());
    }
}