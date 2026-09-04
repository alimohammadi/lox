// parser.rs

use crate::tokenize::Tokens;

pub type AST = ();

pub fn parse(token: Tokens) -> AST {
    println!("Parsing");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }
}