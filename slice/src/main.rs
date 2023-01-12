/*Q->Write a function that takes a string of words separated by spaces and returns 
the first word it finds in that string. If the function doesn’t find a space in the 
string, the whole string must be one word, so the entire string should be 
returned. (use slice)*/

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
