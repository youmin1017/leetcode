class Solution:
    def search(self, nums: list[int], target: int) -> int:
        l, r = 0, len(nums)-1
        mid = (l+r) // 2

        while l < r and nums[mid] != target:
            if nums[mid] > target:
                r = mid - 1
            elif nums[mid] < target:
                l = mid + 1
            mid = (l + r) // 2

        if nums[mid] == target:
            return mid
        else:
            return -1
