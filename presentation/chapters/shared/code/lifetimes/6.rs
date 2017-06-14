use std::str::Split;

struct Tokenizer<'i> {
    input: Split<'i, char>,
}

impl<'i> Tokenizer<'i> {
    fn next_token(&mut self) -> Option<&'i str> {
        self.input.next()
    }
}

struct Parser<'t, 'i: 't> {
    tokenizer: &'t mut Tokenizer<'i>,
}

impl<'t, 'i: 't> Parser<'t, 'i> {
    fn next_item(&mut self) -> Option<&'i str> {
        self.tokenizer.next_token()
    }
}

fn main() {
    let mut tok = Tokenizer { input: "( foo bar )".split(' ') };
    let mut parser = Parser { tokenizer: &mut tok };

    println!("{:?}", parser.next_item());
    let content = parser.next_item();
    let content2 = parser.next_item();
    println!("{:?}", parser.next_item());
    drop(parser);

    println!("{:?}", content);
    println!("{:?}", content2);

}