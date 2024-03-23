// use lexer::Token::*;
use std::fmt;

#[derive(PartialOrd, PartialEq, Clone)]
pub enum Sexp {
    Atom(String),
    Cons(Box<Sexp>, Box<Sexp>),
    Nil
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
    // use super::*;
    use super::Sexp;
    use super::Sexp::*;

    fn nil() -> Box<Sexp> {
        Box::new(Nil)
    }

    fn atom(s: &str) -> Box<Sexp> {
        Box::new(Atom(String::from(s)))
    }

    fn cons(first: Box<Sexp>, rest: Box<Sexp>) -> Box<Sexp> {
        Box::new(Cons(first, rest))
    }

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
    }
}
