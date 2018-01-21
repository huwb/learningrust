use threadpool::ThreadPool;
use std::sync::Arc;
use std::sync::Mutex;

pub fn run() {
    let x = 20000;

    ::std::thread::sleep(::std::time::Duration::from_millis(2000));
    println!("ST:");
    let results = primes_up_to(x);
    println!("{:?}", results);

    ::std::thread::sleep(::std::time::Duration::from_millis(2000));
    println!("MT:");
    let results = primes_up_to_mt(x, 4);
    println!("{:?}", results);


    // let mut results_sorted = results.clone();
    // results_sorted.sort();
    // assert_eq!(results, results_sorted);


    // let x = 100000008;

    // println!("Is {} prime? Computing..", x);

    // let prime = is_prime(x);

    // if prime {
    //     println!("{} is prime!", x);
    // } else {
    //     println!("{} is not prime, factors:", x);
    //     println!("\t{:?}", factors(x));
    // }

    // println!("Primes up to {}:", x);
    // println!("{:?}", primes_up_to(x));
}

pub fn is_prime(x: i32) -> bool {
    factors(x).len() <= 2 && x > 1
}

pub fn primes_up_to(x: i32) -> Vec<i32> {
    assert!(x >= 0);
    let mut results = vec![];
    for i in 2..x + 1 {
        if is_prime(i) {
            results.push(i);
        }
    }
    results
}

pub fn primes_up_to_mt(x: i32, thread_count: usize) -> Vec<i32> {
    assert!(x >= 0);

    let tp = ThreadPool::new(thread_count);
    let factors = Arc::new(Mutex::new(vec![]));

    for i in 2..x + 1 {
        let arc = Arc::clone(&factors);
        tp.dispatch(Box::new(move || {
            if is_prime(i) {
                (*arc.lock().unwrap()).push(i);
            }
        }));
    }

    drop(tp);

    // retrieve out of reference and mutex
    let mut results: Vec<i32> = (*factors.lock().unwrap())
        .iter()
        .map(|item| *item)
        .collect();
    
    // no guarantee that the results come back in order
    results.sort();
    results
}

pub fn factors(x: i32) -> Vec<i32> {
    assert!(x >= 0);

    let mut results = vec![];

    for factor in 1..x + 1 {
        if x % factor == 0 {
            results.push(factor);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factors_of_zero_are_none() {
        assert_eq!(factors(0), vec![]);
    }

    #[test]
    fn factors_of_one_is_one() {
        assert_eq!(factors(1), vec![1]);
    }

    #[test]
    fn factors_of_two_is_one_and_two() {
        assert_eq!(factors(2), vec![1, 2]);
    }

    #[test]
    fn factors_three_to_six() {
        assert_eq!(factors(3), vec![1, 3]);
        assert_eq!(factors(4), vec![1, 2, 4]);
        assert_eq!(factors(5), vec![1, 5]);
        assert_eq!(factors(6), vec![1, 2, 3, 6]);
    }

    #[test]
    fn zero_not_prime() {
        assert!(!is_prime(0));
    }

    #[test]
    fn one_not_prime() {
        assert!(!is_prime(1));
    }

    #[test]
    fn prime_examples() {
        assert!(is_prime(2));
        assert!(is_prime(7));
    }

    #[test]
    fn nonprime_examples() {
        assert!(!is_prime(4));
        assert!(!is_prime(6));
    }

    #[test]
    fn compute_primes_st_vs_mt() {
        let x = 1000;
        assert_eq!(primes_up_to(x), primes_up_to_mt(x, 4));
    }
}
