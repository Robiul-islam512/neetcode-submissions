impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut flags = [false;1000];

    let mut strs_freq:Vec<HashMap<char,i32>> = Vec::new();

    for word in strs.iter(){
        let mut freq:HashMap<char,i32> = HashMap::new();
        for ch in word.chars(){
            freq.entry(ch).and_modify(|count|*count+=1).or_insert(1);
        }
        strs_freq.push(freq);
    }

    let mut left = 0 as usize;
    let mut right  = 1 as usize;

    let mut res:Vec<Vec<String>> = Vec::new();

    while right <= strs.len() {
        let mut inside:Vec<String> = Vec::new();

        for i in right..strs_freq.len(){
            if strs_freq[left] == strs_freq[i] && !flags[i]{
                flags[i] = true;
                inside.push(strs[i].clone());
            }
        }
        if !flags[left]{
            inside.push(strs[left].clone());
            res.push(inside);
            flags[left] = true;

        }
        
        left+=1;
        right = left+1;
    }
    res
    }
}
