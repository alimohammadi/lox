// reader.rs
// read source code from a file


// This is type alias. Note: "()" is the Rust "unit" type. It's kinda like None in Python.
pub type Source = ();
pub type Error = ();

pub fn read_source(filename: &str) -> Result<Source, Error> {
    println!("Reading source code");

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