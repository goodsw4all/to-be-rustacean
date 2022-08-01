// https://leetcode.com/problems/merge-sorted-array/

pub fn merge_3_loops(nums1: &Vec<i32>, m: i32, nums2: &Vec<i32>, n: i32) -> Vec<i32> {
    let mut merged_array = vec![];

    let mut idx1: usize = 0;
    let mut idx2: usize = 0;
    while idx1 < m as usize && idx2 < n as usize {
        if nums1[idx1] <= nums2[idx2] {
            merged_array.push(nums1[idx1]);
            idx1 += 1;
        } else {
            merged_array.push(nums2[idx2]);
            idx2 += 1;
        }
    }

    while idx1 < m as usize {
        merged_array.push(nums1[idx1]);
        idx1 += 1;
    }

    while idx2 < n as usize {
        merged_array.push(nums2[idx2]);
        idx2 += 1;
    }

    merged_array
}

pub fn merge_1_loop(nums1: &Vec<i32>, m: i32, nums2: &Vec<i32>, n: i32) -> Vec<i32> {
    let mut merged_array = vec![];

    let mut idx1: usize = 0;
    let mut idx2: usize = 0;

    if m == 0 {
        return nums2.to_vec();
    }
    if n == 0 {
        return nums1.to_vec();
    }

    loop {
        if idx1 == m as usize && idx2 == n as usize {
            break;
        }
        if idx1 < m as usize && (idx2 == n as usize || nums1[idx1] <= nums2[idx2]) {
            merged_array.push(nums1[idx1]);
            idx1 += 1;
        } else {
            merged_array.push(nums2[idx2]);
            idx2 += 1;
        }
    }

    merged_array
}

pub fn merge_rusty_extend(nums1: &Vec<i32>, m: i32, nums2: &Vec<i32>, n: i32) -> Vec<i32> {
    let mut merged_array = vec![];
    merged_array.extend(&nums1[0..m as usize]);
    merged_array.extend(&nums2[0..n as usize]);

    merged_array.sort();
    merged_array
}

pub fn merge_rusty_concat(nums1: &Vec<i32>, m: i32, nums2: &Vec<i32>, n: i32) -> Vec<i32> {
    let mut merged_array = [&nums1[0..m as usize], &nums2[0..n as usize]].concat();
    merged_array.sort();
    merged_array
}

pub fn merge(nums1: &Vec<i32>, m: i32, nums2: &Vec<i32>, n: i32) -> Vec<i32> {
    // merge_3_loops(nums1, m, nums2, n)
    // merge_1_loop(nums1, m, nums2, n)
    // merge_rusty_extend(nums1, m, nums2, n)
    merge_rusty_concat(nums1, m, nums2, n)
}

#[test]
fn test_merge_sorted_array() {
    assert_eq!(merge(&vec![], 0, &vec![], 0), []);

    assert_eq!(merge(&vec![], 0, &vec![1, 4, 7], 3), [1, 4, 7]);

    assert_eq!(merge(&vec![1, 4, 7], 3, &vec![], 0), [1, 4, 7]);

    let nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let nums2 = vec![2, 5, 6];
    let n = 3;
    assert_eq!(merge(&nums1, m, &nums2, n), [1, 2, 2, 3, 5, 6]);

    assert_eq!(merge(&nums2, n, &nums1, m), [1, 2, 2, 3, 5, 6]);
}
