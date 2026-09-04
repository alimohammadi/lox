// tokenize.rs

use crate::reader::Source;

pub type Tokens = ();

pub fn tokenize(source: Source) -> Tokens {
    println!("Tokenizing source code");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let source = Source::new("print \"Hello, world!\"");
        let tokens = tokenize(source);
        assert_eq!(tokens, vec!["print", "Hello, world!"]);
    }
}