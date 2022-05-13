pub fn _max_sub_array(nums: Vec<i32>) -> i32 {
    let (mut cur, mut max) = (nums[0], nums[0]);

    for i in 1..nums.len() {
        cur = match cur < 0 {
            true => nums[i],
            _ => cur + nums[i],
        };
        max = max.max(cur);
    }
    max
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn works() {
        let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
        assert_eq!(_max_sub_array(nums), 6);
    }
}
