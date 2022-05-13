pub fn _selection_sort(nums: &mut Vec<i32>) -> &Vec<i32> {
    for i in 0..nums.len() {
        let mut min = nums[i];
        let mut index = i;
        let mut j = i + 1;
        while j < nums.len() {
            if nums[j] < min {
                min = nums[j];
                index = j;
            }
            j += 1;
        }
        nums.swap(i, index);
    }
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_sort_test() {
        let mut t = vec![1, 3, 3, 4, 6, 2, 5, 2];
        assert_eq!(_selection_sort(&mut t), &[1, 2, 3, 3, 4, 5, 6]);
    }
}
