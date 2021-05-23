using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

// Source : https://leetcode.com/problems/two-sum/
// Author : Yue Lu
// Date   : 2021-05-23

namespace _1_two_sum
{
    class Solution
    {
        public int[] TwoSum(int[] nums, int target)
        {
            int[] ret = new int[2];
            Dictionary<int, int> arrayDict = new Dictionary<int, int>();
            for (int id = 0; id < nums.Length; id++)
            {
                // dynamic judge
                if (arrayDict.ContainsKey(target - nums[id]))
                {
                    BuildResult(arrayDict[target - nums[id]], id, ret);
                    return ret;
                }
                if (!arrayDict.ContainsKey(nums[id]))
                {
                    arrayDict.Add(nums[id], id);
                }
            }
            return ret;
        }

        private void BuildResult(int index1, int index2, int[] ret)
        {
            ret[0] = index1;
            ret[1] = index2;
        }
    }
}
