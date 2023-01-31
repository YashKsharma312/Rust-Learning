
#[derive(Debug)]
enum states{
    albama,
    newyork,
    ohio,
    atlanta
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(states),
}


fn main() {
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(states) =>{
                println!("Quarter is of {:?} state. ",states);
                25
            }
             
       }
}
let a=value_in_cents(Coin::Penny);
let b=value_in_cents(Coin::Quarter(states::atlanta));
println!("{}",a);
println!("{}",b);

//convert option<u32> to u32

fn convert(x: Option<u32>) -> u32 {
    match x {
        None => 0,
        Some(u) => u,
    }
}
println!("{}",convert(Some(5)));

//Dice hat game
fn dice(dice_roll:u8)->(){
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}
}

fn add_fancy_hat() {
    println!("Congratulation you unlocked a hat!!");
}
fn remove_fancy_hat() {
    println!("Unfortunately your hat has been ceased!!");
}
fn move_player(num_spaces: u8) {
    println!("Move your player with {num_spaces} stepes!!");
}

println!("{:?}",dice(3));
println!("{:?}",dice(7));
println!("{:?}",dice(5));
}
