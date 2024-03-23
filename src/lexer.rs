use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Atom(char),
    Op(char),
    EOF
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let chars = input.chars().peekable();
        Lexer {chars}
    }

    pub fn next(self: &mut Self) -> Token {
        self.skip_ws();
        if let Some(c) = self.chars.next() {
            return Lexer::token(c)
        }
        Token::EOF
    }

    pub fn peek(self: &mut Self) -> Token {
        self.skip_ws();
        if let Some(c) = self.chars.peek() {
            return Lexer::token(*c)
        }
        Token::EOF
    }

    pub fn token(c: char) -> Token {
        match c {
            '0' ..= '9' |
            'a' ..= 'z' | 'A' ..= 'Z' => {
                Token::Atom(c)
            },
            _ => Token::Op(c)
        }
    }

    pub fn skip_ws(self: &mut Self) {
        while self.chars.peek().map_or(false, |s| s.is_ascii_whitespace()) {
            self.chars.next();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut l1 = Lexer::new("foo bar");
        
        assert_eq!(l1.peek(), Token::Atom('f'));
        assert_eq!(l1.next(), Token::Atom('f'));
        assert_eq!(l1.next(), Token::Atom('o'));
        assert_eq!(l1.next(), Token::Atom('o'));
        assert_eq!(l1.next(), Token::Atom('b'));
        assert_eq!(l1.next(), Token::Atom('a'));
        assert_eq!(l1.next(), Token::Atom('r'));
        assert_eq!(l1.peek(), Token::EOF);
        assert_eq!(l1.next(), Token::EOF);
    }

    #[test]
    fn it_works_for_string_too() {
        let s = String::from("xx yy");
        let mut l2 = Lexer::new(&s);

        assert_eq!(l2.next(), Token::Atom('x'));
        assert_eq!(l2.next(), Token::Atom('x'));
        assert_eq!(l2.peek(), Token::Atom('y'));
        assert_eq!(l2.next(), Token::Atom('y'));
        assert_eq!(l2.next(), Token::Atom('y'));
    }
}
