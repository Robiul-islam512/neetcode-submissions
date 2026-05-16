use std::collections::HashMap;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut freqCount:HashMap<i32,i32> = HashMap::new();

        for val in nums{
            freqCount.entry(val).and_modify(|count|*count+=1).or_insert(1);
        }

        for (_,v) in freqCount{
            if v>=2{
                return true
            }
        }
        false
    }
}
