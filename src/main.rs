use std::io;
use fern::lex;
use fern::parse;

fn main() {
    //let mut buf = String::new();
    //io::stdin().read_to_string(&mut buf);
    match io::read_to_string(io::stdin()) {
        Err(e) => (),
        Ok(contents) => match lex::lex(contents).and_then(|mut x| parse::parse_aux(&mut x)) {
            Err(e) => (),
            Ok(ast) => println!("{:?}", ast),
        }
    }
}
