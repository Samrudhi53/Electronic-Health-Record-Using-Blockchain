fn find_first_occurrence(nums: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = nums.len();

    while low < high {
        let mid = low + (high - low) / 2;
        if nums[mid] < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    if low < nums.len() && nums[low] == target {
        Some(low)
    } else {
        None
    }
}

fn main() {
    let nums = vec![1, 2, 4, 4, 4, 5, 6, 7];
    let target = 4;
    match find_first_occurrence(&nums, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}
