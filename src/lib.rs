pub mod generic_fn;
pub mod closures;
pub mod iterators;
pub mod smart_pointers;
pub mod points;

pub fn run() {
    let hello;
    let longest;
    {
        hello = "hello";
        let goodbye = "goodbye";
        longest = longest_string(hello, goodbye);
    }

    println!("Longest: {}", longest);

    let list = [4, 5, 3, 1, 2];
    let largest = generic_fn::_largest_in_list(&list);
    println!("Largest: {}", largest);

    points::run();
    // closures::run();
    smart_pointers::run();
}

/// Returns the larger of the two strings
///
/// # Examples
///
/// ```
/// use learningrust::longest_string;
///
/// fn main() {
///     assert_eq!( longest_string("short", "long"), "short");
/// }
/// ```
pub fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_empty_strings() {
        let x = "";
        let y = "";
        let longest = longest_string(x, y);
        assert_eq!(longest, x);
    }
}
