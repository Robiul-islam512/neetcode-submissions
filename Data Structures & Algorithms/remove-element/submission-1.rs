impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = nums.len();
        while i>0{
            i-=1;

            if nums[i as usize] == val{
                nums.swap_remove(i);
            }
        }
        nums.len() as i32
    }
}
