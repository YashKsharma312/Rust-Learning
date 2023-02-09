#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn main() {
    let r;
    let x = 5;
    r = &x;
    println!("r: {}", r);
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    let string11 = String::from("efghijklmnopqrstuvwxyz");
    let string22 = "abcd";

    let result = longesta(string11.as_str(), string22);
    println!("The longest string is {}", result);

    let novel = String::from("Call me yash. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}",i);
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longesta<'a>(x: &'a str, y: &str) -> &'a str {
    x
}




