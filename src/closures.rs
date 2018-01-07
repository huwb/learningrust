use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn() -> usize,
{
    calculation: T,
    value: Option<usize>,
}

impl<T> Cacher<T>
where
    T: Fn() -> usize,
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
