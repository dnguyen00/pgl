use lexer::Lexer;
use syntax::Syntax;

mod lexer;
mod tokens;
mod syntax;

fn main() {
    let str = "if(x 
        > 20 || y <= 40 && v >= z
    ) 
    { hello 
        + 
        world; 
    }
     if(
        x>20
    ){hello
        +world
        ;
    }{1230+
        1231;
        5+23213;hello+
        world;}";
    //let str = "{1230+1231;5+23213;hello+world;}";
    //let str = "hello + world * 1249129.521512";
    let lexer = Lexer::new(str);
    lexer.clone().parse();

    let mut syntax = Syntax::new(lexer);
    println!("{:?}", syntax.check_validity());
}