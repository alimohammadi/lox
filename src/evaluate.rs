// evaluate.rs

use crate::parser::AST;


// This is type alias. Note: "()" is the Rust "unit" type. It's kinda like None in Python.
pub type Output = ();

pub fn evaluate(ast: AST) -> Output {
    println!("Evaluating");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }
}