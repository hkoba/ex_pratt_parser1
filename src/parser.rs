use crate::lexer::*;
use crate::sexp::*;

pub fn expr(input: &str) -> Result<Box<Sexp>, String> {
    let mut lexer = Lexer::new(input);

    expr_bp(&mut lexer)
}

fn expr_bp(lexer: &mut Lexer) -> Result<Box<Sexp>, String> {
    let lhs = match lexer.next() {
        Token::Atom(c) => {
            atom(c.to_string().as_str())
        },
        c => {
            return Err(format!("atom is expected: {:?}", c))
        }
    };

    loop {
        let op = match lexer.next() {
            Token::EOF => break,
            Token::Op(c) => c,
            t => return Err(format!("operator is expected: {:?}", t)),
        };

        todo!();
    }

    Ok(lhs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = expr("1").unwrap();
        assert_eq!(s.to_string(), "1");
        // assert_eq!(expr("1 + 2 * 3").unwrap().to_string(), "(+ 1 (* 2 3))")
    }
}
