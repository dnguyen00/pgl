use lexer::Lexer;

mod lexer;
mod tokens;
mod syntax;

fn main() {
    let mut lexer = Lexer::new("==!=;{}");
    lexer.parse();
}