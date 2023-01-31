extern crate movie_lib;
use movie_lib::movies::play;
fn main(){
    println!("Inside main of test");
    play("harry potter".to_string())
}