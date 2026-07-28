use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut n = nums.len();
        let mut freq:HashMap<i32,i32> = HashMap::new();

        for num in nums.iter(){
            freq.entry(*num).and_modify(|count|*count+=1).or_insert(1);
        }
        let mut mx = 0;
        let mut mxVal = 0;
        for num in nums{
            if *freq.get(&num).unwrap()>mx {
                mx = *freq.get(&num).unwrap();
                mxVal = num;
            }
        }
        mxVal
    }
}
