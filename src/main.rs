extern crate learningrust;

fn main() {

    let hello;
    let longest;
    {
        hello = "hello";
        let goodbye = "goodbye";
        longest = learningrust::longest_string(hello, goodbye);
    }

    println!("Longest: {}", longest);

    let list = [4, 5, 3, 1, 2];
    let largest = learningrust::generic_fn::_largest_in_list(&list);
    println!("Largest: {}", largest);
}
