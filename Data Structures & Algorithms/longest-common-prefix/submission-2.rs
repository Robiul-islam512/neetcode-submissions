impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
    
    let mut minLenWord = strs[0].clone();

    for word in strs.iter(){
        if word.len()<=minLenWord.len(){
            minLenWord = word.clone()
        }
    }

    let mut left = 0 as usize;
    let mut right = strs.len();

    while left<right {
        if !strs[left].contains(&minLenWord){
            minLenWord.pop();
        }
        else{
            left+=1;
        }
    }

    minLenWord
    }
}
