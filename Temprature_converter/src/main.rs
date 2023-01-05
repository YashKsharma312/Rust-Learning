use std::io;

fn main() {
    println!("Temprature conversion!!");

    println!("please input your temprature in celcius");
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("failed to read line");
        println!("your temp : {temp}");
        let mut temp: u32 = temp.trim().parse().expect("Please type a number!");

        let fah=((temp*9)/5)+32;
        println!("temprature in fahrenhit= : {fah}");
       
   
} 