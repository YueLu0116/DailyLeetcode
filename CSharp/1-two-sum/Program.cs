using System;

namespace _1_two_sum
{
    class Program
    {
        static void Main(string[] args)
        {
            int[] nums = new int[] { 3,3 };
            int target = 6;
            Solution mySolution = new Solution();
            int[] result = mySolution.TwoSum(nums, target);
            foreach(var n in result)
            {
                Console.WriteLine(n);
            }
        }
    }
}
