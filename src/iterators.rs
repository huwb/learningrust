#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn iterator_sum() {
        let v1 = vec![4, 9, 2];
        let iter = v1.iter();
        let total: i32 = iter.sum();

        assert_eq!(total, 15);
    }

    #[test]
    fn iterator_adaptor_map() {
        let v1 = vec![2, 6, 3, 4];

        let v2: Vec<_> = v1.iter().map(|item| item + 3).collect();
        assert_eq!(v2, vec![5, 9, 6, 7]);
    }

    #[test]
    fn iterator_adaptor_filter() {
        let v1: Vec<i32> = vec![9, 8, 3, 4, 1];
        let v1_even: Vec<i32> = v1.into_iter().filter(|item| item % 2 == 0).collect();
        assert_eq!(v1_even, vec![8, 4]);
    }
}
