

// break the project into four parts
// Each in their own file

// Discussion: Use of "mod"
// Coming from python, these "mod" declarations might look similar to "import" statements. 
// How ever something else is really going on. Instead of importing code, "mod" is declaring
// the existence of submodule and is required so that the Rust compiler can actually find the code in other files.
// You only need to specify "mod" in one place (usually main.rs) and nowhere else in the program.
mod reader;
mod tokenize;
mod parser;
mod evaluate;

fn main() {
    println!("Hello, world!");

    let source = reader::read_source("somefile.lox");

    let tokens = tokenize::tokenize(source);

    let ast = parser::parse(tokens);

    let out = evaluate::evaluate(ast);
}
 