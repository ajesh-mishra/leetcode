from tkinter.tix import Tree
from typing import List, Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        if not root:
            return []
        
        result = []
        Solution.inner(root, result)
        return result


    def inner(root, result):
        if root.left: Solution.inner(root.left, result)
        result.append(root.val)            
        if root.right: Solution.inner(root.right, result)


if __name__ == '__main__':
    s = Solution()
    result = s.inorderTraversal(t)
    print(result)