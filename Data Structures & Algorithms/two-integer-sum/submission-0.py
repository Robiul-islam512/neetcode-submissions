class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        n = len(nums)
        l,r= 0,1
        count = 0
        while(l<r):
            if r<n and nums[l]+nums[r] == target:
                return [l,r]
            else:
                count+=1
                r+=1
            if count == n-1:
                l+=1
                r = l+1
                count = 0