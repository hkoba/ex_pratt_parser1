// use lexer::Token::*;
use std::fmt;

#[derive(PartialOrd, PartialEq, Clone)]
pub enum Sexp {
    Atom(String),
    Cons(Box<Sexp>, Box<Sexp>),
    Nil
}

use Sexp::*;

pub fn nil() -> Box<Sexp> {
    Box::new(Nil)
}

pub fn atom(s: &str) -> Box<Sexp> {
    Box::new(Atom(String::from(s)))
}

pub fn cons(first: Box<Sexp>, rest: Box<Sexp>) -> Box<Sexp> {
    Box::new(Cons(first, rest))
}

impl fmt::Display for Sexp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sexp::Nil => {
              write!(f, "()")
            },
            Sexp::Atom(s) => {
                write!(f, "{}", s)
            },
            Sexp::Cons(first, rest) => {
                write!(f, "({}", *first)?;
                let mut p = rest;
                loop {
                    match &**p {
                        Sexp::Nil => {
                            break
                        }
                        Sexp::Atom(s) => {
                            write!(f, " . {}", s)?;
                            break
                        },
                        Sexp::Cons(fst, rst) => {
                            write!(f, " {}", *fst)?;
                            p = &rst;
                        }
                    }
                }
                write!(f, ")")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disp1() {
        assert_eq!(Nil.to_string(), "()");

        assert_eq!(cons(atom("foo"), nil()).to_string(), "(foo)");

        assert_eq!(Cons(atom("foo")
                        , cons(atom("bar"), nil())
        ).to_string(), "(foo bar)");

        assert_eq!(Cons(atom("foo")
                        , atom("bar")
        ).to_string(), "(foo . bar)");

        assert_eq!(cons(cons(atom("foo")
                             , cons(atom("bar"), nil()))
                        , cons(cons(atom("baz")
                                    , cons(atom("qux"), nil())), nil())
        ).to_string(), "((foo bar) (baz qux))");

    }
}
