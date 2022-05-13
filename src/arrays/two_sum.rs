use std::collections::HashMap;

// pub fn two_sum_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     let mut h_m = HashMap::<i32, usize>::new();
//     for (index, element) in nums.iter().enumerate() {
//         let expected = target - element;
//         match h_m.get(&expected) {
//             Some(&pre) => return vec![pre as i32, index as i32],
//             _ => {
//                 h_m.insert(*element, index);
//             }
//         }
//     }
//     unreachable!()
// }

pub fn _two_sum_hashmap2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut heap_map = HashMap::new();
    for (index, element) in nums.into_iter().enumerate() {
        let num = target - element;
        match heap_map.get(&num) {
            Some(&pre) => return vec![pre as i32, index as i32],
            _ => {
                heap_map.insert(element, index);
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn works() {
        let nums = vec![1, 2, 3];
        let target = 5;
        assert_eq!(_two_sum_hashmap2(nums, target), [1, 2]);
    }
}
