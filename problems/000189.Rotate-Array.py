class Solution:
    def rotate(self, nums: List[int], k: int) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        po = len(nums) - k % len(nums)
        tmp = nums[0:po]
        for x in tmp:
            nums.append(x)
        del nums[0:po]
        pass
