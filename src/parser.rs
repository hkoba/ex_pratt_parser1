use crate::lexer::*;
use crate::sexp::*;

pub fn expr(input: &str) -> Box<Sexp> {
    let mut lexer = Lexer::new(input);

    expr_bp(&mut lexer)
}

fn expr_bp(lexer: &mut Lexer) -> Box<Sexp> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(expr("1 + 2 * 3").to_string(), "(+ 1 (* 2 3))")
    }
}
