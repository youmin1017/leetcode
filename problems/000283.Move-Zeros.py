class Solution:
    def moveZeroes(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        insert_position = 0
        for i in range(len(nums)): 
            if nums[i] != 0:
                nums[i], nums[insert_position] = nums[insert_position], nums[i]
                insert_position += 1
