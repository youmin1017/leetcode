# The isBadVersion API is already defined for you.
# @param version, an integer
# @return an integer
# def isBadVersion(version):

class Solution:
    def firstBadVersion(self, n):
        """
        :type n: int
        :rtype: int
        """
        l, r, mid = 1, n, (n+1) // 2

        while l < r:
            bad = isBadVersion(mid)
            if bad:
                r = mid - 1
            elif not bad:
                l = mid + 1
            if not isBadVersion(r):
                return r + 1
            mid = (l + r) // 2
        return l
