use std::thread;
use std::time::Duration;

pub fn run() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Cacher<T>
where
    T: FnMut() -> usize,
{
    calculation: T,
    value: Option<usize>,
}

impl<T> Cacher<T>
where
    T: FnMut() -> usize,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self) -> usize {
        match self.value {
            Some(result) => result,
            None => {
                let v = (self.calculation)();
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn generate_workout(intensity: usize, random_number: usize) {
    let mut calc_result = Cacher::new(|| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", calc_result.value());
        println!("Next, do {} situps!", calc_result.value());
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", calc_result.value());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cacher_starts_empty() {
        let cacher = Cacher::new(|| 0);
        // before calling value(), calculation result should be None
        assert_eq!(cacher.value, None);
    }

    #[test]
    fn cacher_first_call_populates() {
        let mut closure_calls = 0;
        let calc_result = 2;
        let v;
        {
            let closure = || {
                closure_calls += 1;
                calc_result
            };

            let mut cacher = Cacher::new(closure);

            v = cacher.value();
        }

        // calculation should have been executed once
        assert_eq!(closure_calls, 1);

        // and returned the right result
        assert_eq!(v, calc_result);
    }

    #[test]
    fn cacher_second_call_uses_cache() {
        let mut closure_calls = 0;
        {
            let closure = || {
                closure_calls += 1;
                0
            };

            let mut cacher = Cacher::new(closure);

            // value retrieved twice
            cacher.value();
            cacher.value();
        }

        // calculation should have only happened once
        assert_eq!(closure_calls, 1);
    }
}
