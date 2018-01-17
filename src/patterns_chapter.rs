pub fn run() {
    println!("Literals");
    let x = 8;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!(">two"),
    };

    println!("Named variables, multiple patterns, ranges");
    let x = Some(2);
    match x {
        Some(5) | Some(6) => println!("It's a five-or-sixer!"),
        Some(y @ 1...3) => println!("It's a thricer: {}", y),
        Some(y) => println!("It's a {}er!", y),
        _ => println!("It's anotherer"),
    }

    println!("Destructuring structs");
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    // voodoo
    let Point { x, y } = p;
    if let Point { x: 0, y: _ } = p {
        println!("Point lies on x=0!");
    }
    println!("a = {}, b = {}", x, y);

    println!("Destructuring references");
    let vectors = vec![
        Point { x: 3, y: 4 },
        Point { x: 1, y: 5 },
        Point { x: 2, y: 7 },
    ];
    let sum_of_square_lengths: i32 = vectors.iter().map(|&Point { x, y }| x * x + y * y).sum();
    println!("The sum is: {}", sum_of_square_lengths);

    println!("Ignore remaining parts of pattern");
    let (.., x, _) = (5, 6, 7, 9);
    println!("Only matched x={}", x);

    println!("Create references in patterns");
    let robot_name = Some(String::from("Bors"));
    match robot_name {
        // the syntax is not simply Some(&name) - that would destructure a reference as shown above!
        Some(ref name) => println!("Found a name: {}", *name),
        None => (),
    }
    println!("robot_name is: {:?}", robot_name);
}
