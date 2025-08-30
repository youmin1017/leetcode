/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        if(head == nullptr or head->next == nullptr) {
            return nullptr;
        }
        ListNode* pNode = head;
        vector<ListNode*> NodePtrs;

        while(pNode != nullptr) {
            NodePtrs.emplace_back(pNode);
            pNode = pNode->next;
        }
        if(n == 1) {
            (*(NodePtrs.end() - 2))->next = nullptr;
        } else {
            **(NodePtrs.end() - (n)) = **(NodePtrs.end() - n + 1);
        }
        return head;
    }
};
