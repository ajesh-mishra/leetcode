from typing import List


class Solution:
    def findRestaurant(self, list1: List[str], list2: List[str]) -> List[str]:
        min = 10_000
        result = list()

        for i, l1 in enumerate(list1):
            if l1 in list2:
                j = list2.index(l1)
                if i+j < min:
                    min = i+j
                    result = list()
                    result.append(l1)
                elif i+j == min:
                    result.append(l1)

        return result
        

if __name__ == '__main__':
    list1 = ["Shogun","Tapioca Express","Burger King","KFC"]
    list2 = ["Piatti","The Grill at Torrey Pines","Hungry Hunter Steakhouse","Shogun"]
    
    s = Solution()
    print(s.findRestaurant(list1, list2))