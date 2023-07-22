use std::io;
use fern::lex;
use fern::parse;

fn main() {
    match io::read_to_string(io::stdin()) {
        Err(e) => eprintln!("Error reading from stdin: {e}"),
        Ok(contents) => match lex::lex(contents).and_then(|mut x| parse::parse(&mut x)) {
            Err(e) => eprintln!("{}", e),
            Ok(ast) => println!("{:?}", ast),
        }
    }
}
