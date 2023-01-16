/*Given an integer array nums, return true if any value appears at 
least twice in the array, and return false if every element is distinct.*/

fn main(){
    //Method 1
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut result=false;
        let mut arr=vec![];
        for items in nums{
            for item in &arr{
                if items==*item{
                result=true;
                }
            }
            arr.push(items);
        }
        result
  
    }


//Method 2
    pub fn contains_duplicate1(nums: Vec<i32>) -> bool {
        let mut result=false;
        for (pos, element) in nums.iter().enumerate(){
            for (pos1, element1) in nums.iter().enumerate(){
                if pos == pos1 {
                    continue;
                }
                else if element1==element{
                    result=true;
                }
            }
        }
        result    
    }
    let a=contains_duplicate([2,5,7,11].to_vec());
    println!("{a}");
}
