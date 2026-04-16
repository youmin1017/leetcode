// Cpoy from rainingpeace
class Solution {
public:
    bool checkInclusion(string s1, string s2) {
        vector<int> freq(26); 
        for (auto& ch : s1) freq[ch - 'a']++; 
        for (int i = 0; i < s2.size(); ++i) {
            freq[s2[i]-'a']--; 
            if (i >= s1.size()) freq[s2[i-s1.size()]-'a']++; 
            if (all_of(freq.begin(), freq.end(), [](int x){return x == 0;})) return true; 
        }
        return false; 
    }
};
