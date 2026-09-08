// evaluate.rs

use crate::parser::AST;


// This is type alias. Note: "()" is the Rust "unit" type. It's kinda like None in Python.
pub type Output = ();
pub type Error = ();

pub fn evaluate(ast: AST) -> Result<Output, Error> {
    println!("Evaluating");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }
}