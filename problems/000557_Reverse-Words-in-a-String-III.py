class Solution:
    def reverseWords(self, s: str) -> str:
        strs = s.split(' ')
        res = ''
        for i, str in enumerate(strs):
            res += str[::-1]
            if i != len(strs) - 1:
                res += ' '
        return res
