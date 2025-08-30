class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        # Two Pointer left and right
        l, r = 0, len(nums) - 1
        while nums[l] + nums[r] != target:
            if nums[l] + nums[r] > target:
                r -= 1
            else:
                l += 1
        return (l+1, r+1)
