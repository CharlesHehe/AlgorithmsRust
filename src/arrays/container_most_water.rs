use std::cmp;
pub fn max_area_traverse(height: Vec<i32>) -> i32 {
    let length = height.len();
    let mut max = 0;
    let mut cur = 0;
    for i in 0..(length - 1) {
        for j in (i + 1)..(length) {
            match cmp::min(height.get(j), height.get(i)) {
                Some(x) => {
                    cur = x * ((j - i) as i32);
                    max = cmp::max(max, cur);
                }
                None => print!("Null"),
            }
        }
    }
    max
}

pub fn max_area(height: Vec<i32>)->i32{
    // let sorted = height.sort();
    1

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_area_test() {
        let t = vec![1,1];
        assert_eq!(max_area_traverse(t), 3);
    }
}
