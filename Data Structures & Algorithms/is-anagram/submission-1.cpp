class Solution {
public:
    bool isAnagram(string s, string t) {
        unordered_map<char,int>hash(26);

        s.erase(remove(s.begin(),s.end(),' '),s.end());
        t.erase(remove(t.begin(),t.end(),' '),t.end());
        if(s.size()!=t.size()){
            return false;
        }
        for(auto c:s){
            hash[c]++;
        }
        for(auto c:t){
            hash[c]--;
            if(hash[c]<0){
                return false;
            }
        }
        return true;
    }
};
