use std::io;
use rand::Rng; // Rng is a trait
use std::cmp::Ordering; //Ordering is enum with three values: "Less", "Greater" "Equal"
use colored::*;
fn main() {
    println!("guessing Game!!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);// gen_range is defined in Rng Trait
    //println!("The secret number is: {secret_number}");
    

    loop {
        println!("please input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");
        
        println!("you guessed : {guess}");
    
        let mut guess: u32 = guess.trim().parse().expect("Please type a number!");// trim removes whitespace entered by user while giving input
        // let  guess: u32 = match guess.trim().parse() {
        //     Ok(num:!)=>num,
        //     Err(_)=>continue,
        // };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            }// end of ordering
           } //end of match
       
    }// end of loop
} // end of main
