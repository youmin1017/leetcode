## Problem

https://leetcode.com/problems/delete-node-in-a-bst/

## Description
```
Given a root node reference of a BST and a key, delete the node with the given key in the BST. Return the root node reference (possibly updated) of the BST.

Basically, the deletion can be divided into two stages:

Search for a node to remove.
If the node is found, delete the node.
Follow up: Can you solve it with time complexity O(height of tree)?
```

Example 1:
![](https://assets.leetcode.com/uploads/2020/09/04/del_node_1.jpg)

## Solution
```
 If the node's val is equal to key, then there are three cases having to be handled.
 	1. both of the node's child node are NULL
 	2. one of child node is NULL
 	3. None of child's node are NULL
 	Target : The node need catch node that has been updated.
 	=> Then, using recursive method { node->left = deleteNode(node->left, key);
                                   { node->right = deleteNode(node->right, key);	
 	Then, start dealing with the three cases
 		1. both of the node's child node are NULL
 			It represents that the previous node should receive the nullptr,
 			since the node have no child node and have been deleted.
 		2. Then,  one of child node is NULL
         O              O      T : target == key
        / \            / \
       T   R    ==>   O   R   ==>   Hence, you should return the existing one
      / \   \          \   \
     O  NULL          NULL
 		3. Third, find the number bigger than root.val and it is the smallest in 
 			 the right sub trees.
     => converting root.val to the number
     => delete the node in the right tree that key == the number, 
           since the node have been moved.
```
## Code
```python
class Solution:
    def deleteNode(self, root: TreeNode, key: int) -> TreeNode:
        if not root :
            return None
        elif key < root.val:
            root.left = self.deleteNode(root.left, key)
        elif key > root.val:
            root.right = self.deleteNode(root.right, key)     
        elif root.val == key :
            if not root.left :
                return root.right
            if not root.right :
                return root.left
            else :
                root.val = self.findmin(root.right)
                root.right = self.deleteNode(root.right, root.val)       
        return root
        
    def findmin(self, node : TreeNode) -> int :
        while node.left :
            node = node.left
        return node.val
```

[Solution implemented by C++](https://gist.github.com/youmin1017/4a38fa05a8f5e1fc5ca03eee6b5e53f5)
