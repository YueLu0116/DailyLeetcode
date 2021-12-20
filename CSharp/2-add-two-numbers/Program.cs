using System;

namespace _2_add_two_numbers
{
    class Program
    {
        static void PrintListFromInputNode(ListNode node)
        {
            ListNode currentNode = node;
            while (currentNode != null)
            {
                Console.WriteLine(currentNode.val);
                currentNode = currentNode.next;
            }
        }

        static void Main(string[] args)
        {
            int[] array1 = new int[] { 8 };
            int[] array2 = new int[] { 9};
            LinkList list1 = new LinkList();
            LinkList list2 = new LinkList();

            foreach (var n in array1)
            {
                list1.AddNode(n);
            }
            Console.WriteLine("List 1:");
            PrintListFromInputNode(list1.GetFristNode());

            foreach (var n in array2)
            {
                list2.AddNode(n);
            }
            Console.WriteLine("List 2:");
            PrintListFromInputNode(list2.GetFristNode());

            Solution sol = new Solution();
            ListNode node = sol.AddTwoNumbers(list1.GetFristNode(), list2.GetFristNode());
            Console.WriteLine("Result:");
            PrintListFromInputNode(node);
        }
    }
}
