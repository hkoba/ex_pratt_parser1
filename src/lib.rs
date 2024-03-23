use std::str::Chars;
use std::iter::Peekable;

struct Lexer<'a> {
    chars: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let chars = input.chars().peekable();
        Lexer {chars}
    }
    pub fn next(self: &mut Self) -> Option<char> {
        self.chars.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut l1 = Lexer::new("foo bar");
        
        assert_eq!(l1.next(), Some('f'));
    }
}
