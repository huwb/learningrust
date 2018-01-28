use std::fmt;

struct Wrapper<T>(Vec<T>);

impl<T> fmt::Display for Wrapper<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[").unwrap();
        if self.0.len() > 0 {
            write!(f, "{}", self.0[0]).unwrap();
            for elem in self.0.iter().skip(1) {
                write!(f, ", {}", elem).unwrap();
            }
        }
        write!(f, "]")
    }
}

pub fn run() {
    let sv = vec![String::from("Yo"), String::from("Dude")];
    let iv = vec![1, 2];
    let nv: Vec<i32> = vec![];

    println!("{}", Wrapper(sv));
    println!("{}", Wrapper(iv));
    println!("{}", Wrapper(nv));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_vector() {
        let nv: Vec<i32> = vec![];
        assert_eq!(Wrapper(nv).to_string(), "[]");
    }

    #[test]
    fn int_vector_1() {
        let iv = vec![4];
        assert_eq!(Wrapper(iv).to_string(), "[4]");
    }

    #[test]
    fn int_vector_2() {
        let iv = vec![4, 3];
        assert_eq!(Wrapper(iv).to_string(), "[4, 3]");
    }

    #[test]
    fn string_vector_3() {
        let iv = vec![String::from("Yo"), String::from("what\'s"), String::from("Up")];
        assert_eq!(Wrapper(iv).to_string(), "[Yo, what\'s, Up]");
    }
}
