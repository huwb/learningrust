pub trait OutlinePrint {
    fn outline_print(&self);
}

impl OutlinePrint for str {
    fn outline_print(&self) {
        let len = self.len();
        let stars = "*".repeat(len + 4);
        println!("{}", stars);
        println!("* {} *", self);
        println!("{}", stars);
    }
}
