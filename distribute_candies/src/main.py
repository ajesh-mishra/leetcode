from typing import Set


class Solution(object):
    def distributeCandies(self, candyType):
        return min(len(candyType) / 2, len(set(candyType)))


if __name__ == '__main__':
    s = Solution()
    print(s.distributeCandies([1,1,2,2,3,3]))