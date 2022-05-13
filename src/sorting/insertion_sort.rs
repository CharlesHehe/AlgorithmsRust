pub fn _insertion_sort(nums: &mut Vec<i32>) -> &Vec<i32> {
    for i in 1..nums.len() {
        let mut j = i;
        while j > 0 && nums[j - 1] > nums[j] {
            nums.swap(j - 1, j);
            j -= 1;
        }
    }
    nums
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insertion_sort_test() {
        let mut t = vec![1, 3, 2];
        assert_eq!(_insertion_sort(&mut t), &[1, 2, 3]);
    }
}
