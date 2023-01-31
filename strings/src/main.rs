fn main() {
    let hello = String::from("नमस्ते");
    println!("{hello}");
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");
    let s1 = String::from("Hello ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{s3}");
    let s_1 = String::from("tic");
    let s_2 = String::from("tac");
    let s_3 = String::from("toe");

    let s4 = format!("{s_1}-{s_2}-{s_3}");
    println!("{s4}");
}
