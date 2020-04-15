fn main() {
    assert_eq!(binary_search(vec![1, 2, 3, 4, 5, 8], 10), -1);
    assert_eq!(binary_search(vec![1, 2, 3, 4, 5, 8], 1), 0);
}

fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 0 {
        return -1;
    }
    
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    -1
}