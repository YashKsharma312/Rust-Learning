/*Q:- You are given an array prices where prices[i] is the price of a given stock on the ith day.
You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.
Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.*/

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
