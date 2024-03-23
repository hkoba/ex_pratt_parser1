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
        while self.chars.peek().map_or(false, |s| s.is_ascii_whitespace()) {
            self.chars.next();
        }
        if let Some(c) = self.chars.next() {
            return match c {
                '0' ..= '9' |
                'a' ..= 'z' | 'A' ..= 'Z' => {
                    Token::Atom(c)
                },
                _ => Token::Op(c)
            }
        }
        Token::EOF
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut l1 = Lexer::new("foo bar");
        
        assert_eq!(l1.next(), Token::Atom('f'));
        assert_eq!(l1.next(), Token::Atom('o'));
        assert_eq!(l1.next(), Token::Atom('o'));
        assert_eq!(l1.next(), Token::Atom('b'));
        assert_eq!(l1.next(), Token::Atom('a'));
        assert_eq!(l1.next(), Token::Atom('r'));
        assert_eq!(l1.next(), Token::EOF);
    }

    #[test]
    fn it_works_for_string_too() {
        let s = String::from("xx yy");
        let mut l2 = Lexer::new(&s);

        assert_eq!(l2.next(), Token::Atom('x'));
        assert_eq!(l2.next(), Token::Atom('x'));
        assert_eq!(l2.next(), Token::Atom('y'));
        assert_eq!(l2.next(), Token::Atom('y'));
    }
}
