use std::io;

fn main() {
    println!("Fabonnaci series!!");

    println!("please input your number");
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("failed to read line");
        let mut n: u32 = n.trim().parse().expect("Please type a number!");
        let mut n1=0;
        let mut n2=1;
        let mut next=n1+n2;
        println!("{}
{}", n1, n2);
        for i in 3..n+1 {
            println!("{}", next);
            n1=n2;
            n2=next;
            next=n1+n2;

        }

}