class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        start, maxlen = 0, 0
        map = {}

        for i, c in enumerate(s):
            if map.get(c) != None and map[c] >= start:
                start = map[c] + 1
            map[c] = i
            maxlen = max(i-start+1, maxlen)
        return maxlen
