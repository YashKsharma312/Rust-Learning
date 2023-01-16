#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}",user.email);

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("{}",user1.email);

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    let mut user2=build_user("user2@haha.com".to_string(),"user2username".to_string());
    println!("{}",user2.email);

    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("user3.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user4 = User {
        email: String::from("user4.com"),
        ..user2
    };

    println!("{}",user3.email);
    println!("{}",user4.email);
    println!("{}",user3.active);
    println!("{}",user4.active);
    println!("user4 is {:?}", user4);

    //tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);


    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}",black.1);
    println!("{}",origin.0);
}
