// fn main() {
//     println!("Hello, world!");
// }

fn sum(left: i32, right: i32) -> i32 {
    left + right
}
fn main() {
    let a = 42;
    let b = 1;
    let s = sum(a, b);
    println!("this sum of {} and {} is {}", a, b, s); // no error!
}