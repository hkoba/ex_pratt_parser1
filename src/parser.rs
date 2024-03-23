use crate::lexer::*;
use crate::sexp::*;

pub fn expr(input: &str) -> Result<Box<Sexp>, String> {
    let mut lexer = Lexer::new(input);

    expr_bp(&mut lexer, 0)
}

fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> Result<Box<Sexp>, String> {
    let mut lhs = match lexer.next() {
        Token::EOF => nil(),
        Token::Atom(c) => {
            atom(c.to_string().as_str())
        },
        Token::Op('(') => {
            let lhs = expr_bp(lexer, 0).unwrap();
            assert_eq!(lexer.next(), Token::Op(')'));
            lhs
        },
        Token::Op(c) => {
            if let Some(((), r_bp)) = prefix_binding_power(c) {
                let rhs = expr_bp(lexer, r_bp).unwrap();
                cons(atom(c.to_string().as_ref()), cons(rhs, nil()))
            } else {
                nil()
            }
        }
    };

    loop {
        let op = match lexer.peek() {
            Token::EOF => break,
            Token::Op(c) => c,
            t => return Err(format!("operator is expected: {:?}", t)),
        };

        if let Some((l_bp, ())) = postfix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();

            lhs = if op == '[' {
                let rhs = expr_bp(lexer, 0).unwrap();
                assert_eq!(lexer.next(), Token::Op(']'));
                cons(atom(op.to_string().as_str())
                     , cons(lhs
                            ,cons(rhs, nil())))
            } else {
                cons(atom(op.to_string().as_str())
                     , cons(lhs, nil()))
            };

            continue;
        }

        if let Some((l_bp, r_bp)) = infix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();

            let rhs = expr_bp(lexer, r_bp).unwrap();

            lhs = cons(atom(op.to_string().as_str())
                       , cons(lhs
                              , cons(rhs, nil())));

            continue;
        }

        break;
    }

    Ok(lhs)
}

fn infix_binding_power(c: char) -> Option<(u8, u8)> {
    match c {
        '+' | '-' => Some((1, 2)),
        '*' | '/' => Some((3, 4)),
        '・' => Some((10, 9)),
        _ => None
    }
}

fn prefix_binding_power(c: char) -> Option<((), u8)> {
    match c {
        '+' | '-' => Some(((), 5)),
        _ => None
    }
}

fn postfix_binding_power(c: char) -> Option<(u8, ())> {
    match c {
        '!' | '[' => Some((7, ())),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = expr("").unwrap();
        assert_eq!(s.to_string(), "()");

        let s = expr("1").unwrap();
        assert_eq!(s.to_string(), "1");

        let s = expr("1 + 2 * 3").unwrap();

        // cargo test -- --nocapture
        println!("s is:\n{:#?}", *s);

        assert_eq!(s.to_string(), "(+ 1 (* 2 3))");

        let s = expr("a + b * c * d + e").unwrap();
        assert_eq!(s.to_string(), "(+ (+ a (* (* b c) d)) e)");

        let s = expr("f ・ g ・ h").unwrap();
        assert_eq!(s.to_string(), "(・ f (・ g h))");

        let s = expr("--1 * 2").unwrap();
        assert_eq!(s.to_string(), "(* (- (- 1)) 2)");

        let s = expr("--f ・ g").unwrap();
        assert_eq!(s.to_string(), "(- (- (・ f g)))");

        let s = expr("-9!").unwrap();
        assert_eq!(s.to_string(), "(- (! 9))");

        let s = expr("f ・ g !").unwrap();
        assert_eq!(s.to_string(), "(! (・ f g))");

        let s = expr("(((0)))").unwrap();
        assert_eq!(s.to_string(), "0");

        let s = expr("x[0][1]").unwrap();
        assert_eq!(s.to_string(), "([ ([ x 0) 1)");
    }
}
