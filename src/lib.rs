pub mod generic_fn;
pub mod closures;
pub mod iterators;

pub fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[test]
fn test_longest_string() {
    let x = "short";
    let y = "long";
    let longest = longest_string(x, y);
    assert_eq!(longest, x);
}
