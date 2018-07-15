use std::io;

fn main() {
    println!("Please input key");

    let mut key = String::new();
    io::stdin().read_line(&mut key)
        .expect("Failed to read line");
    
    println!("You key: {}", key);
}
