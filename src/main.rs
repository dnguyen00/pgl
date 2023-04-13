use std::fs;

use lexer::Lexer;
use syntax::Syntax;

mod lexer;
mod tokens;
mod syntax;
mod tests;

fn main() {
    let file = fs::read_to_string("lexer.txt");
    if file.is_err() {
        panic!("{:?}", file.err());
    }

    let mut file_contents = file.ok().unwrap().as_str().to_owned();

    file_contents = "{if(a>b){hello+world;};}".to_owned();

    let lexer = Lexer::new(&file_contents);
    lexer.clone().parse();

    let mut syntax = Syntax::new(lexer);
    println!("lexer.txt syntax analyzer result: {:?}", syntax.check_validity());
}