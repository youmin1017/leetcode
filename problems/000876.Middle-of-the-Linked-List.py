
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def middleNode(self, head: Optional[ListNode]) -> Optional[ListNode]:
        # 11 22 32 43 53 64 74
        if head.next == None:
            return head
        middle, count = ListNode(head.val, head.next), 0
        while head.next != None:
            if count%2 == 0:
                middle = middle.next
            count += 1
            head = head.next
        return middle
