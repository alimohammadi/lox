// tokenize.rs

use crate::reader::Source;


// Discussion: This is trying to express some "high level" thinking about the problem of tokenizing. First tokenizing will return 
// all of the tokens in some way (not yet known). Second, there is a possiblity that tokenizing will fail(in some way) if given bad input.
// The two type aliases bellow are "stubs" for these two posibilities(But details not yet figured out)

pub struct Tokens {}
pub type Error = ();

// The standard way of handling errors in Rust is returning a Result<T, E> type.    

pub fn tokenize(source: Source) -> Result<Tokens, Error> {
    println!("Tokenizing source code");
    Ok(Tokens {  })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }
}