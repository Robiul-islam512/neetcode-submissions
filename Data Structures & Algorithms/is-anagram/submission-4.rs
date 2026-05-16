use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len()!=t.len(){return false};

        let mut sChars:Vec<char> = s.chars().collect();
        let mut tChars:Vec<char> = t.chars().collect();

        sChars.sort();
        tChars.sort();

        for i in 0..sChars.len(){
            if sChars[i]!=tChars[i]{
                return false
            }
        }
        true
    }
}
