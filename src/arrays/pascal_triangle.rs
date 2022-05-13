pub fn _generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    for x in 0..num_rows as usize {
        res.push(vec![1; x + 1]);
        for y in 1..x {
            res[x][y] = res[x - 1][y - 1] + res[x - 1][y];
        }
    }
    return res;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn works() {
        assert_eq!(
            _generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 6, 4, 1]
            ]
        );
    }
}
