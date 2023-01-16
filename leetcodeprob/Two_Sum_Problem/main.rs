/*  Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
 You may assume that each input would have exactly one solution, and you may not use the same element twice.
 You can return the answer in any order.*/

 //TWO SUM PROBLEM



fn main() {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
         let mut result = vec![0,0];
         for (pos, element) in nums.iter().enumerate(){
            for (pos1, element1) in nums.iter().enumerate(){
                if pos == pos1 {
                    continue;
                }else if element+ element1 == target {
                    result[0]= pos as i32;
                    result[1]= pos1 as i32;
                }
            }
        }
        return result;
    }
    let a=[2,5,7,11].to_vec();
    let b=two_sum(a,7);
    println!("{:?}",b);
}
