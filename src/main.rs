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

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    learningrust::closures::generate_workout(
        simulated_user_specified_value,
        simulated_random_number,
    );
}
