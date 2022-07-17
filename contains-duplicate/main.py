from typing import Set


class Solution:
    def containsDuplicate(self, nums) -> bool:
        if len(nums) == len(set(nums)):
            return False
        else:
            return True

if __name__ == '__main__':
    s = Solution()
    print(s.containsDuplicate([1,2,3,1]))
    print(s.containsDuplicate([1,2,3]))