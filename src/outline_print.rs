use std::fmt::Display;

pub trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        let stars = "*".repeat(len + 4);
        println!("{}", stars);
        println!("* {} *", output);
        println!("{}", stars);
    }
}

impl OutlinePrint for str {}
