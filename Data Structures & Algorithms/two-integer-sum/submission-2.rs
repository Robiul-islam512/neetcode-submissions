use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut freqCount:HashMap<i32,i32> = HashMap::new();

        let n = nums.len();

        for i in 0..n{
            freqCount.entry(nums[i as usize]).or_insert(i as i32);
        }

        let mut ans = vec![];

        for i in 0..n{
            let compare = target-nums[i as usize];
            if let Some(&j) = freqCount.get(&compare){
                if i as usize !=j as usize{
                    ans.push(i as i32);
                    ans.push(j as i32);
                    break;
                }
              
            }
        }
       ans.sort();
       ans
    }
}
