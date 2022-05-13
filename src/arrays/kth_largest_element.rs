use std::collections::BinaryHeap;

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut max_heap = BinaryHeap::from(nums);
    let mut k = k as usize;
    while (k as usize) >1 {
        max_heap.pop();
        k-=1;
    }
    *max_heap.peek().unwrap()
}