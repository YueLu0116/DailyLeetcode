using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

// Source : https://leetcode.com/problems/add-two-numbers/
// Author : Yue Lu
// Date   : 2021-05-25

namespace _2_add_two_numbers
{
    public class ListNode
    {
        public int val;
        public ListNode next;
        public ListNode(int val=0, ListNode next = null)
        {
            this.val = val;
            this.next = next;
        }
    }

    public class LinkList
    {
        private ListNode mHeadNode;

        public LinkList()
        {
            mHeadNode = new ListNode();
        }

        public void AddNode(int val)
        {
            ListNode currentNode = mHeadNode;
            ListNode newNode = new ListNode(val);
            while (currentNode.next != null)
            {
                currentNode = currentNode.next;
            }
            currentNode.next = newNode;
        }



        public ListNode GetFristNode()
        {
            return mHeadNode.next;
        }
    }

    class Solution
    {
        public ListNode AddTwoNumbers(ListNode l1, ListNode l2)
        {
            ListNode result = new ListNode();
            ListNode nodeCruise = result;
            int carrier = 0;
            do
            {
                int currentVal = l1.val + l2.val + carrier;
                carrier = currentVal / 10;
                int units = currentVal % 10;
                ListNode currentNode = new ListNode(units);
                nodeCruise.next = currentNode;
                nodeCruise = nodeCruise.next;
                l1 = l1.next;
                l2 = l2.next;
            } while (l1!=null && l2!=null);

            while (l1 != null)
            {
                int currentVal = l1.val + carrier;
                carrier = currentVal / 10;
                int units = currentVal % 10;
                ListNode currentNode = new ListNode(units);
                nodeCruise.next = currentNode;
                nodeCruise = nodeCruise.next;
                l1 = l1.next;
            }

            while (l2 != null)
            {
                int currentVal = l2.val + carrier;
                carrier = currentVal / 10;
                int units = currentVal % 10;
                ListNode currentNode = new ListNode(units);
                nodeCruise.next = currentNode;
                nodeCruise = nodeCruise.next;
                l2 = l2.next;
            }

            if(carrier != 0)
            {
                ListNode currentNode = new ListNode(carrier);
                nodeCruise.next = currentNode;
                nodeCruise = nodeCruise.next;
            }
            return result.next;
        }
    }
}
