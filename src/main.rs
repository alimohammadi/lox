

// break the project into four parts
// Each in their own file
mod reader;
mod tokenize;
mod parser;
mod evaluate;


fn main() {
    println!("Hello, world!");

    reader::read_source();
    tokenize::tokenize();
    parser::parse();
    evaluate::evaluate();
}
