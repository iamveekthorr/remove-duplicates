fn main() {
    // Given an integer array nums sorted in non-decreasing order, remove some duplicates in-place such that each unique element appears at most twice. The relative order of the elements should be kept the same.

    // Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.

    // Return k after placing the final result in the first k slots of nums.

    let mut nums = vec![1, 1, 1, 2, 2, 3];
    let k = remove_duplicates(&mut nums);
    println!("{}", k);
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    // Your code here
    let mut left = 0;

    for i in 0..nums.len() {
        if left < 2 {
            nums[left] = nums[i];
            left += 1;
        } else if left >= 2 {
            if nums[i] != nums[left - 2] {
                nums[left] = nums[i];
                left += 1;
            }
        }
    }

    return left.try_into().unwrap();
}
