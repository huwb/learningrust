pub struct Counter {
    counter: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { counter: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;
        match self.counter {
            i if i < 6 => Some(i),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_calling_next() {
        let mut c = Counter::new();
        assert_eq!(c.next(), Some(1));
        assert_eq!(c.next(), Some(2));
        assert_eq!(c.next(), Some(3));
        assert_eq!(c.next(), Some(4));
        assert_eq!(c.next(), Some(5));
        assert_eq!(c.next(), None);
    }

    #[test]
    fn counter_other_iterator_traits() {
        let result: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|a| a % 3 == 0)
            .sum();

        assert_eq!(result, 18);
    }

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
