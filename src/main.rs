use lexer::Lexer;
use syntax::Syntax;

mod lexer;
mod tokens;
mod syntax;

fn main() {
    let lexer = Lexer::new("if(x > 20) { hello + world }");
    lexer.clone().parse();

    let mut syntax = Syntax::new(lexer);
    syntax.check_validity();
}