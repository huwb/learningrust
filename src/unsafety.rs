struct WhyHello {
    value: i32,
}

impl WhyHello {
    fn say_it(&self) {
        println!("Why Hellooooooo.:.:.:.{}", self.value);
    }
}

static HELLO_WORLD: &str = "Hello, hello";

static mut WHY_HELLO: WhyHello = WhyHello { value: 2 };

pub fn run() {
    let handle = ::std::thread::spawn(|| unsafe {
        WHY_HELLO.value += 1;
        WHY_HELLO.say_it();
        println!("Hola");
    });
    unsafe {
        WHY_HELLO.value += 1;
    }

    println!("I'm unsafe: {}", HELLO_WORLD);

    unsafe {
        WHY_HELLO.say_it();
    }

    handle.join().unwrap();
}
