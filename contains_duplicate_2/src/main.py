from typing import List


class Solution:
    def containsNearbyDuplicate_1(self, nums: List[int], k: int) -> bool:
        i = 0

        while i + 1 < len(nums):
            for j in range(1, k+1):
                if not i + j < len(nums):
                    break

                if nums[i] == nums[i+j]:
                    return True
            i += 1

        return False


    def containsNearbyDuplicate(self, nums: List[int], k: int) -> bool:
        count = {}

        for i, n in enumerate(nums):
            if n in count.keys():
                count[n].append(i)
            else:
                count[n] = [i]
        
        for c in count.values():
            if len(c) < 2:
                continue

            i = 0
            for i in range(0, len(c) - 1):
                if c[i+1] - c[i] <= k:
                    return True

        return False

                
if __name__ == '__main__':
    nums = [1,2,3,1]
    k = 3
    s = Solution()
    print(s.containsNearbyDuplicate(nums, k))