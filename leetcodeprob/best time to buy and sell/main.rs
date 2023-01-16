fn main(){
   pub fn max_profit(prices: Vec<i32>) -> i32 {
      let mut min_price=i32::pow(10,4);
      let mut profit=0;
      for item in prices{
          if item<min_price{
              min_price=item;
          }
          else if item-min_price>profit{
              profit=item-min_price;
          }
      }
      profit
  }

    let b=[2,5,7,11].to_vec();
    let b1=[7,6,4,3,1].to_vec();
    let b2=[7,5,8,6,1].to_vec();
    let a=max_profit(b);
    let a1=max_profit(b1);
    let a2=max_profit(b2);
    println!("{a},{a1},{a2}");
}
