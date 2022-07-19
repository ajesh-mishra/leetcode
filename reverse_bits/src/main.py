from tokenize import String


class Solution:
    def reverseBits(self, n: int) -> int:
        s = f'{n:032b}'[::-1]
        return int(s, 2)


if __name__ == '__main__':
    s = Solution()
    print(f'{s.reverseBits(11000001)}')
