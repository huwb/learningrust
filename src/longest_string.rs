use generic_fn;

pub fn run() {
    let hello;
    let l;
    {
        hello = "hello";
        let goodbye = "goodbye";
        l  = longest(hello, goodbye);
    }

    println!("Longest: {}", l);

    let list = [4, 5, 3, 1, 2];
    let largest = generic_fn::_largest_in_list(&list);
    println!("Largest: {}", largest);
}

/// Returns the larger of the two strings
///
/// # Examples
///
/// ```
/// use learningrust::longest_string;
///
/// fn main() {
///     assert_eq!( longest_string::longest("short", "long"), "short");
/// }
/// ```
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
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
        let longest = longest(x, y);
        assert_eq!(longest, x);
    }
}
