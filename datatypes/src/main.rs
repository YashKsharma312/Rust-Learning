use std::io;
fn main() {
    let x:u8=254;
    

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;
    println!("Sum {}", sum);

    // subtraction
    let diff = 95.5 - 4.3;
    println!("diff {}", diff);
    // multiplication
    let product = 4 * 30;
    println!("product {}", product);
    // division
    let quotient = 56.7 / 32.2;
    println!("quotient {}", quotient);
    let truncated = -5 / 3; // Results in -1
    println!("truncated {}", truncated);
    // remainder
    let remainder = 43 % 5;
    println!("remainder {}", remainder);

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    let t = true;

    let f: bool = false; // with explicit type annotation
    println!("bool is {}", f);


    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;
    println!("first, second and third tuple values {} {} {}", five_hundred, six_point_four, one);
//Array

let a = [1, 2, 3, 4, 5];

println!("Please enter an array index.");

let mut index = String::new();

io::stdin().read_line(&mut index).expect("Failed to read line");

let index: usize = index.trim().parse().expect("Index entered was not a number");

let element = a[index];

println!("The value of the element at index {index} is: {element}");
}

