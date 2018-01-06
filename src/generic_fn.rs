pub fn _largest_in_list<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for l in list {
        if l > largest {
            largest = &l;
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_in_list_returns_largest_value() {
        let list = [4, 5, 3, 1, 2];
        assert_eq!(*_largest_in_list(&list), 5);
    }

    #[test]
    #[should_panic]
    fn largest_in_empty_list_panics() {
        let v: Vec<usize> = Vec::new();
        assert_eq!(*_largest_in_list(&v[0..0]), 3);
    }
}
