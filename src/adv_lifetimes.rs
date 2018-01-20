pub fn run() {
    let c = Context("hellloooooo");
    let result = parse_context(c);
    println!("{:?}", result);
}

#[derive(Debug)]
struct Context<'s>(&'s str);

struct Parser<'a, 's: 'a> {
    context: &'a Context<'s>,
}

impl<'a, 's> Parser<'a, 's> {
    fn parse(&'a self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

// tricky lifetime - Parser is a temporary struct. we tell
// rust above that the string that Parser returns has a
// greater lifetime using the 's: 'a syntax above.
fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}
