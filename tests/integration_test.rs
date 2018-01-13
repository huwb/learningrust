extern crate learningrust;

#[test]
fn longest_string_works() {
    assert_eq!(learningrust::longest_string::longest("yo", "yoyoyo"), "yoyoyo");
}
