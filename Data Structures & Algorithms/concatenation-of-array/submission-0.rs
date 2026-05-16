impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        let n = nums.len();
        
        for i in 0..2*n{
            ans.push(nums[i%n]);
        }

        ans
    }
}
