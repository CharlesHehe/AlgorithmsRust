fn merge_sort(vec: &Vec<i32>) -> Vec<i32> {
    if vec.len() < 2 {
        vec.to_vec()
    } else {
        let mid = vec.len() / 2;
        let left = merge_sort(&vec[..mid].to_vec());
        let right = merge_sort(&vec[mid..].to_vec());
        merge(&left, &right)
    }
}

fn merge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    let mut merged = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < left.len() && j < right.len() {
        if left[i] >= right[j] {
            merged.push(right[j]);
            j += 1;
        } else {
            merged.push(left[i]);
            i += 1;
        }
    }
    if i < left.len() {
        while i < left.len() {
            merged.push(left[i]);
            i += 1;
        }
    }
    if j < right.len() {
        while j < right.len() {
            merged.push(right[j]);
            j += 1;
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let t = vec![1, 3, 3, 4, 6, 2, 5, 2];
        assert_eq!(merge_sort(&t), vec![1, 2, 2, 3,3, 4, 5, 6]);
    }
}