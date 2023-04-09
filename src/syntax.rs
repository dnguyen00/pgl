use crate::{lexer::{Lexer, self, Lexemes}, tokens::Tokens};

#[derive(Debug)]
pub struct Syntax<'a> {
    lexer: Lexer<'a>,
}

impl Syntax<'_> {
    pub fn new(lexer: Lexer) -> Syntax {
        return Syntax { lexer: lexer };
    }

    pub fn check_validity(&mut self) -> Option<bool> {
        if self.lexer.peek_token() == None {
            return None;
        }

        println!("{:?}", self.parse_stmt());

        return None;
    }

    fn parse_stmt_list(&mut self) -> Option<Grammar> {
        if self.lexer.peek_token().clone() == None {
            return Some(Grammar::STMTLIST);
        }

        loop {
            if self.parse_stmt() != Some(Grammar::STMT) {
                return Some(Grammar::STMTLIST); //THIS MIGHT BE A SPECIAL CASE, I HAVE NO CLUE
            }

            let peek_token = self.lexer.peek_token().clone().unwrap();
            let match_terminal_token = [Tokens::SEMICOLON];
            let mut is_set = false;
            for token in match_terminal_token {
                if peek_token.token == token {
                    self.lexer.next_token();
                    is_set = true;
                    break;
                }
            }

            if !is_set {
                break;
            }
        }
        
        return Some(Grammar::STMTLIST);
    }

    fn parse_stmt(&mut self) -> Option<Grammar> {
        let original_lexer = self.lexer.clone();

        if self.parse_if_stmt() == Some(Grammar::IFSTATEMENT) {
            return Some(Grammar::IFSTATEMENT);
        }

        self.lexer = original_lexer.clone();

        if self.parse_block() == Some(Grammar::BLOCK) {
            return Some(Grammar::BLOCK);
        }

        self.lexer = original_lexer.clone();

        if self.parse_expr() == Some(Grammar::EXPRESSION) {
            return Some(Grammar::EXPRESSION);
        }

        return Some(Grammar::UNKNOWN);
    }

    //need to implement else, ends at (<STMT> `;` | <BLOCK>)
    fn parse_if_stmt(&mut self) -> Option<Grammar> {
        if self.lexer.peek_token().unwrap().clone().token != Tokens::IDENTIFIER {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().unwrap().clone().lexeme.as_str() != "if" {
            return Some(Grammar::UNKNOWN);
        }

        self.lexer.next_token();

        if self.lexer.peek_token().clone().unwrap().token != Tokens::LPARENTHESIS {
            return Some(Grammar::UNKNOWN);
        }
        self.lexer.next_token();

        if self.parse_bool_expr() != Some(Grammar::BOOLEXPRESSION) {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().clone().unwrap().token != Tokens::RPARENTHESIS {
            return Some(Grammar::UNKNOWN);
        }
        self.lexer.next_token();

        let original_lexer = self.lexer.clone();
        if self.parse_stmt() == Some(Grammar::STMT) {
            if self.lexer.peek_token().is_some() {
                if self.lexer.next_token().unwrap().token == Tokens::SEMICOLON {
                    return Some(Grammar::IFSTATEMENT);
                }
            }
        }

        self.lexer = original_lexer;

        if self.parse_block() == Some(Grammar::BLOCK) {
            return Some(Grammar::IFSTATEMENT);
        }

        return Some(Grammar::UNKNOWN);
    }

    fn parse_block(&mut self) -> Option<Grammar> {
        if self.lexer.peek_token().clone() == None {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.next_token().clone().unwrap().token != Tokens::LBRACKET {
            return Some(Grammar::UNKNOWN);
        }

        if self.parse_stmt_list() != Some(Grammar::STMTLIST) {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().clone() == None {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.next_token().clone().unwrap().token != Tokens::RBRACKET {
            return Some(Grammar::UNKNOWN);
        }

        return Some(Grammar::BLOCK);
    }

    fn parse_bool_expr(&mut self) -> Option<Grammar> {
        if self.parse_bterm() != Some(Grammar::BTERM) {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().clone() == None {
            return Some(Grammar::BOOLEXPRESSION);
        }

        loop {
            let peek_token = self.lexer.peek_token().clone().unwrap();
            let match_terminal_token = [Tokens::GREAT, Tokens::LESS, Tokens::GREATEQ, Tokens::LESSEQ];
            let mut is_set = false;
            for token in match_terminal_token {
                if peek_token.token == token {
                    self.lexer.next_token();
                    
                    if self.parse_bterm() != Some(Grammar::BTERM) {
                        return Some(Grammar::UNKNOWN);
                    }
    
                    is_set = true;
                    break;
                }
            }

            if !is_set {
                break;
            }
        }
        
        return Some(Grammar::BOOLEXPRESSION);
    }

    fn parse_bterm(&mut self) -> Option<Grammar> {
        if self.parse_band() != Some(Grammar::BAND) {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().clone() == None {
            return Some(Grammar::BTERM);
        }

        loop {
            let peek_token = self.lexer.peek_token().clone().unwrap();
            let match_terminal_token = [Tokens::EQUALITY, Tokens::INEQUALITY];
            let mut is_set = false;
            for token in match_terminal_token {
                if peek_token.token == token {
                    self.lexer.next_token();
                    
                    if self.parse_band() != Some(Grammar::BAND) {
                        return Some(Grammar::UNKNOWN);
                    }
    
                    is_set = true;
                    break;
                }
            }

            if !is_set {
                break;
            }
        }
        
        return Some(Grammar::BTERM);
    }

    fn parse_band(&mut self) -> Option<Grammar> {
        if self.parse_bor() != Some(Grammar::BOR) {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().clone() == None {
            return Some(Grammar::BAND);
        }

        loop {
            let peek_token = self.lexer.peek_token().clone().unwrap();
            let match_terminal_token = [Tokens::AND];
            let mut is_set = false;
            for token in match_terminal_token {
                if peek_token.token == token {
                    self.lexer.next_token();
                    
                    if self.parse_bor() != Some(Grammar::BOR) {
                        return Some(Grammar::UNKNOWN);
                    }
    
                    is_set = true;
                    break;
                }
            }

            if !is_set {
                break;
            }
        }
        
        return Some(Grammar::BAND);
    }

    fn parse_bor(&mut self) -> Option<Grammar> {
        if self.parse_expr() != Some(Grammar::EXPRESSION) {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().clone() == None {
            return Some(Grammar::BOR);
        }

        loop {
            let peek_token = self.lexer.peek_token().clone().unwrap();
            let match_terminal_token = [Tokens::AND];
            let mut is_set = false;
            for token in match_terminal_token {
                if peek_token.token == token {
                    self.lexer.next_token();
                    
                    if self.parse_expr() != Some(Grammar::EXPRESSION) {
                        return Some(Grammar::UNKNOWN);
                    }
    
                    is_set = true;
                    break;
                }
            }

            if !is_set {
                break;
            }
        }
        
        return Some(Grammar::BOR);
    }

    fn parse_expr(&mut self) -> Option<Grammar> {
        if self.parse_term() != Some(Grammar::TERM) {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().clone() == None {
            return Some(Grammar::EXPRESSION);
        }

        loop {
            let peek_token = self.lexer.peek_token().clone().unwrap();
            let match_terminal_token = [Tokens::ADD, Tokens::SUB];
            let mut is_set = false;
            for token in match_terminal_token {
                if peek_token.token == token {
                    self.lexer.next_token();
                    
                    if self.parse_term() != Some(Grammar::TERM) {
                        return Some(Grammar::UNKNOWN);
                    }
    
                    is_set = true;
                    break;
                }
            }

            if !is_set {
                break;
            }
        }

        return Some(Grammar::EXPRESSION);
    }

    fn parse_term(&mut self) -> Option<Grammar> {
        if self.parse_fact() != Some(Grammar::FACT) {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().clone() == None {
            return Some(Grammar::TERM);
        }

        loop {
            let peek_token = self.lexer.peek_token().clone().unwrap();
            let match_terminal_token = [Tokens::MUL, Tokens::DIV, Tokens::MOD];
            let mut is_set = false;
            for token in match_terminal_token {
                if peek_token.token == token {
                    self.lexer.next_token();
    
                    if self.parse_fact() != Some(Grammar::FACT) {
                        return Some(Grammar::UNKNOWN);
                    }
    
                    is_set = true;
                    break;
                }
            }

            if !is_set {
                break;
            }
        }

        return Some(Grammar::TERM);
    }

    fn parse_fact(&mut self) -> Option<Grammar> {
        if self.lexer.peek_token() == None {
            return Some(Grammar::UNKNOWN);
        }

        let mut next_lexeme = self.lexer.next_token().clone().unwrap();

        let match_terminal_token = [Tokens::IDENTIFIER, Tokens::INTEGERS, Tokens::FLOATS];
        for token in match_terminal_token {
            if next_lexeme.token == token {
                return Some(Grammar::FACT);
            }
        }

        if next_lexeme.token != Tokens::LPARENTHESIS {
            return Some(Grammar::UNKNOWN);
        }

        if self.parse_expr() != Some(Grammar::EXPRESSION) {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().is_some() {
            next_lexeme = self.lexer.next_token().clone().unwrap();

            if next_lexeme.token == Tokens::RPARENTHESIS {
                return Some(Grammar::FACT);
            }
        }

        return Some(Grammar::UNKNOWN);
    }
}

#[derive(Debug, PartialEq)]
enum Grammar {
    STMTLIST,
    STMT,
    WHILESTATEMENT,
    IFSTATEMENT,
    BLOCK,
    EXPRESSION,
    TERM,
    FACT,
    BOOLEXPRESSION,
    BTERM,
    BAND,
    BOR,
    UNKNOWN,
}