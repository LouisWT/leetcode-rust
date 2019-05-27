/**
 * [81] Search in Rotated Sorted Array II
 *
 * Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
 * 
 * (i.e., [0,0,1,2,2,5,6] might become [2,5,6,0,0,1,2]).
 * 
 * You are given a target value to search. If found in the array return true, otherwise return false.
 * 
 * Example 1:
 * 
 * 
 * Input: nums = [2,5,6,0,0,1,2], target = 0
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: nums = [2,5,6,0,0,1,2], target = 3
 * Output: false
 * 
 * Follow up:
 * 
 * 
 * 	This is a follow up problem to <a href="/problems/search-in-rotated-sorted-array/description/">Search in Rotated Sorted Array</a>, where nums may contain duplicates.
 * 	Would this affect the run-time complexity? How and why?
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
    if nums.len() == 0 {
        return false;
    }
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let mut mid = left + (right - left) / 2;
        if nums[mid] == target {
        return true;
        }
        // 左边有序了
        if nums[left] < nums[mid] {
        // 如果 target 比左边最小的值还小
        // 或者 target 比左边序列目前的最大值还大
        // left = mid + 1;
        if (nums[left] <= target) && (nums[mid] >= target) {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
        } else if nums[left] > nums[mid] {
        if (target >= nums[mid]) && (target <= nums[right]) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
        } else {
        left += 1;
        }
    }
    return false;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_81() {
        assert_eq!(Solution::search(vec![2,5,6,0,0,1,2], 0), true);
        assert_eq!(Solution::search(vec![2,5,6,0,0,1,2], 3), false);
    }
}
