class Solution:
    def sortedSquares(self, nums: List[int]) -> List[int]:
        res = [ x**2 for x in nums if x >= 0 ]

        for x in nums:
            if x < 0:
                # searchInsert: Find insert position
                res.insert(self.searchInsert(res, x**2), x**2)
            else:
                break
        return res

    def searchInsert(self, nums: list[int], target: int) -> int:
        l, r = 0, len(nums)-1
        mid = (l+r) // 2

        while l <= r:
            if nums[mid] > target:
                r = mid - 1
            elif nums[mid] < target:
                l = mid + 1

            if nums[mid] == target:
                return mid
            mid = (l + r) // 2
        return l
