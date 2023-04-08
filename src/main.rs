use lexer::Lexer;

mod lexer;
mod tokens;
mod syntax;

fn main() {
    let mut lexer = Lexer::new("+-*/%()+=-=/=*=%==<<=>>=&&||hello=12345=1234.555");
    lexer.parse();
}