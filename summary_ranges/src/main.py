from unittest import result


class Solution:
    def summaryRanges(self, nums):
        if len(nums) == 0:
            return []
        else:
            nums
            
        result = []
        temp = []

        for i in nums:
            if (not temp) or (temp[-1] + 1 == i):
                temp.append(i)
                continue

            if len(temp) == 1:
                result.append(f'{temp[0]}')
            else:
                result.append(f'{temp[0]}->{temp[-1]}')

            temp = [i]

        if len(temp) == 1:
            result.append(f'{temp[0]}')
        else:
            result.append(f'{temp[0]}->{temp[-1]}')
        
        return result


if __name__ == '__main__':
    s = Solution()
    print(s.summaryRanges([0, 1, 2, 4, 5, 7]))
    print(s.summaryRanges([0, 2, 3, 4, 6, 8, 9]))