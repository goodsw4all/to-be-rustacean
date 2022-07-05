use std::collections::HashMap;

// https://leetcode.com/problems/two-sum/
// Given an array of integers nums and an integer target,
// return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution,
// and you may not use the same element twice.
// You can return the answer in any order.

// Example 1:

// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].

// O(N^2) O(1)
pub fn two_sum_brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for idx in 0..nums.len() {
        let start_idx = idx + 1;
        let adding_num_idx = nums[start_idx..]
            .iter()
            .position(|x| x == &(target - nums[idx]));

        if let Some(n) = adding_num_idx {
            return vec![idx as i32, (start_idx + n) as i32];
        };
    }
    vec![]
}

// O(N)
pub fn two_sum_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for idx in 0..nums.len() {
        map.insert(nums[idx], idx);
    }
    for idx1 in 0..nums.len() {
        let search_num = target - nums[idx1];
        if let Some(idx2) = map.get(&search_num) {
            if idx1 != *idx2 {
                return vec![idx1 as i32, *idx2 as i32];
            }
        }
    }
    vec![]
}

pub fn two_sum_hashmap2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // k: adding num, v: idx
    let mut map: HashMap<i32, usize> = HashMap::new();
    for idx in 0..nums.len() {
        let search_num = map.get(&(target - nums[idx]));
        if let Some(search_num) = search_num {
            return vec![*search_num as i32, idx as i32];
        } else {
            map.insert(nums[idx], idx);
        }
    }

    vec![]
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // two_sum_brute_force(nums, target)
    // two_sum_hashmap(nums, target)
    two_sum_hashmap2(nums, target)
}

// #[cfg(test)]
#[test]
fn test_two_sum() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(two_sum(nums, target), [0, 1]);

    let nums = vec![2, 7, 11, 15];
    let target = 30;
    assert_eq!(two_sum(nums, target), []);

    let nums = vec![3, 2, 4];
    let target = 6;
    assert_eq!(two_sum(nums, target), [1, 2]);

    let nums = vec![3, 3];
    let target = 6;
    assert_eq!(two_sum(nums, target), [0, 1]);
}
