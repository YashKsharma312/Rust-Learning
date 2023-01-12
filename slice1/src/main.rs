use std::io;
fn main() {
    println!("Enter your string");
    let mut YourString = String::new();
    io::stdin().read_line(&mut YourString).expect("failed to read line");

    let bytes = YourString.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("First word={}",&YourString[0..i]);
        }
}}