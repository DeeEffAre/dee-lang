use crate::lexer::{Lexer, Symbol, Token, TokenType};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    PrefixUnary(UnaryOp, Box<Expr>),
    PostfixUnary(Box<Expr>, UnaryOp),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
    Ident(Symbol),
    FunctionCall(Box<Expr>, Vec<Expr>), // function name, args
    FieldAccess(Box<Expr>, Symbol),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Block(Vec<Statement>),
    Expr(Expr),
    If(Expr, Box<Statement>, Option<Box<Statement>>),
    While(Expr, Box<Statement>),
    DoWhile(Box<Statement>, Expr),
    Return(Option<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Assign,
    Or,
    And,
    BitwiseOr,
    BitwiseAnd,
    BitwiseXor,
    Eq,
    NotEq,
    LessThen,
    GreaterThen,
    LessThenOrEq,
    GreaterThenOrEq,
    RightShift,
    LeftShift,
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    BitwiseOrAssign,
    BitwiseAndAssign,
    ModuloAssign,
    BitwiseXorAssign,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Neg,
    LogicalNot,
    BitwiseNot,
    PreDecrement,
    PreIncrement,
    PostDecrement,
    PostIncrement,
}

impl BinaryOp {
    fn from_token(token: &Token) -> Option<Self> {
        match token.kind {
            TokenType::TokenAssign => Some(BinaryOp::Assign),
            TokenType::TokenDoublePipe => Some(BinaryOp::Or),
            TokenType::TokenDoubleAmpersand => Some(BinaryOp::And),
            TokenType::TokenPipe => Some(BinaryOp::BitwiseOr),
            TokenType::TokenCarret => Some(BinaryOp::BitwiseXor),
            TokenType::TokenAmpersand => Some(BinaryOp::BitwiseAnd),
            TokenType::TokenEqual => Some(BinaryOp::Eq),
            TokenType::TokenBangEqual => Some(BinaryOp::NotEq),
            TokenType::TokenOpenTag => Some(BinaryOp::LessThen),
            TokenType::TokenCloseTag => Some(BinaryOp::GreaterThen),
            TokenType::TokenLessEqual => Some(BinaryOp::LessThenOrEq),
            TokenType::TokenGreaterEqual => Some(BinaryOp::GreaterThenOrEq),
            TokenType::TokenBifShiftRight => Some(BinaryOp::RightShift),
            TokenType::TokenBifShiftLeft => Some(BinaryOp::LeftShift),
            TokenType::TokenPlus => Some(BinaryOp::Add),
            TokenType::TokenMinus => Some(BinaryOp::Sub),
            TokenType::TokenAsterisk => Some(BinaryOp::Mul),
            TokenType::TokenSlash => Some(BinaryOp::Div),
            TokenType::TokenModulo => Some(BinaryOp::Modulo),
            TokenType::TokenPlusAssign => Some(BinaryOp::AddAssign),
            TokenType::TokenMinusAssign => Some(BinaryOp::SubAssign),
            TokenType::TokenAsteriskAssign => Some(BinaryOp::MulAssign),
            TokenType::TokenSlashAssign => Some(BinaryOp::DivAssign),
            TokenType::TokenCarretAssign => Some(BinaryOp::BitwiseXorAssign),
            TokenType::TokenModuloAssign => Some(BinaryOp::ModuloAssign),
            TokenType::TokenAmpersandAssign => Some(BinaryOp::BitwiseAndAssign),
            TokenType::TokenPipeAssign => Some(BinaryOp::BitwiseOrAssign),
            _ => None,
        }
    }
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
    peeked_token: Token,
}

impl<'a> Parser<'a> {
    #[must_use]
    pub fn init_parser(lexer: &'a mut Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::default(),
            peeked_token: Token::default(),
        };

        parser.advance();
        parser.advance();
        parser
    }

    fn current_token(&self) -> &Token {
        &self.current_token
    }

    fn peeked_token(&self) -> &Token {
        &self.peeked_token
    }

    fn advance(&mut self) {
        self.current_token = std::mem::replace(&mut self.peeked_token, self.lexer.next_token())
    }

    fn parse_statement(&mut self) -> Statement {
        match self.current_token().kind {
            TokenType::TokenOpenBrace => self.parse_block().expect("Expected block"), // TODO: proper error handling
            TokenType::TokenIf => self.parse_if().expect("Expected if block"),
            TokenType::TokenWhile => self.parse_while().expect("Expected while block"),
            TokenType::TokenDo => self.parse_do_while().expect("Expected return statement"),
            TokenType::TokenReturn => self.parse_return().expect("Expected return statement"),
            _ => {
                let expr = self.parse_expression(0);
                if matches!(self.current_token().kind, TokenType::TokenSemicolon) {
                    self.advance();
                }
                Statement::Expr(expr)
            }
        }
    }

    fn parse_block(&mut self) -> Option<Statement> {
        self.advance(); // {

        let mut statements = vec![];
        while self.current_token().kind != TokenType::TokenCloseBrace
            && self.current_token().kind != TokenType::TokenEOF
        {
            statements.push(self.parse_statement());
        }

        if self.current_token().kind == TokenType::TokenEOF {
            return None;
        }

        self.advance(); // }
        Some(Statement::Block(statements))
    }

    fn parse_if(&mut self) -> Option<Statement> {
        self.advance(); // if 

        if self.current_token().kind != TokenType::TokenOpenParen {
            return None;
        }
        self.advance(); // (

        let cond = self.parse_expression(0);

        if self.current_token().kind == TokenType::TokenEOF {
            return None;
        }

        self.advance(); // )

        let if_block = self.parse_statement();

        if self.current_token().kind == TokenType::TokenElse {
            self.advance(); // else
            let else_branch = if self.current_token().kind == TokenType::TokenIf {
                self.parse_if().expect("expected if after else") // else if
            } else {
                self.parse_statement()
            };
            Some(Statement::If(
                cond,
                Box::new(if_block),
                Some(Box::new(else_branch)),
            ))
        } else {
            Some(Statement::If(cond, Box::new(if_block), None))
        }
    }

    fn parse_while(&mut self) -> Option<Statement> {
        self.advance(); // while

        if self.current_token().kind != TokenType::TokenOpenParen {
            return None;
        }
        self.advance(); // (

        let cond = self.parse_expression(0);

        if self.current_token().kind == TokenType::TokenEOF {
            return None;
        }
        self.advance(); // )

        let while_block = self.parse_statement();

        Some(Statement::While(cond, Box::new(while_block)))
    }

    fn parse_do_while(&mut self) -> Option<Statement> {
        self.advance(); // do

        let do_block = self.parse_statement();

        if self.current_token().kind != TokenType::TokenWhile {
            return None;
        }
        self.advance(); // while

        if self.current_token().kind != TokenType::TokenOpenParen {
            return None;
        }
        self.advance(); // (

        let cond = self.parse_expression(0);

        if self.current_token().kind == TokenType::TokenEOF {
            return None;
        }
        self.advance(); // )

        Some(Statement::DoWhile(Box::new(do_block), cond))
    }

    fn parse_return(&mut self) -> Option<Statement> {
        self.advance(); // return
        let return_expr = self.parse_expression(0);
        Some(Statement::Return(Some(return_expr)))
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        let token = self.current_token();

        let expr = match token.kind {
            TokenType::TokenIntConstant(val) => Some(Expr::Int(val)),
            TokenType::TokenFloatConstant(val) => Some(Expr::Float(val)),
            TokenType::TokenUIntConstant(val) => Some(Expr::Int(val as i64)),
            TokenType::TokenOpenParen => {
                self.advance();
                let inner = self.parse_expression(0);
                if !(self.current_token().kind == TokenType::TokenCloseParen) {
                    panic!("Expected ')'");
                }
                Some(inner)
            }

            TokenType::TokenIdentifier(sym) => Some(Expr::Ident(sym)),
            _ => None,
        };
        self.advance();

        expr
    }

    pub fn parse_expression(&mut self, min_bp: u8) -> Expr {
        // prefix check
        let mut left = match self.current_token().kind {
            TokenType::TokenDecrement => {
                self.advance();
                let right = self.parse_expression(70);
                Expr::PrefixUnary(UnaryOp::PreDecrement, Box::new(right))
            }
            TokenType::TokenIncrement => {
                self.advance();
                let right = self.parse_expression(70);
                Expr::PrefixUnary(UnaryOp::PreIncrement, Box::new(right))
            }
            TokenType::TokenMinus => {
                self.advance();
                let right = self.parse_expression(70);
                Expr::PrefixUnary(UnaryOp::Neg, Box::new(right))
            }
            TokenType::TokenBang => {
                self.advance();
                let right = self.parse_expression(70);
                Expr::PrefixUnary(UnaryOp::LogicalNot, Box::new(right))
            }
            TokenType::TokenTilde => {
                self.advance();
                let right = self.parse_expression(70);
                Expr::PrefixUnary(UnaryOp::BitwiseNot, Box::new(right))
            }
            _ => self.parse_primary().expect("expected expression"), // TODO: proper error printing
        };

        loop {
            let tok = self.current_token();

            // postfix check
            match tok.kind {
                TokenType::TokenIncrement => {
                    self.advance();
                    left = Expr::PostfixUnary(Box::new(left), UnaryOp::PostIncrement);
                    continue;
                }
                TokenType::TokenDecrement => {
                    self.advance();
                    left = Expr::PostfixUnary(Box::new(left), UnaryOp::PostDecrement);
                    continue;
                }
                _ => (),
            }

            // ternary
            if tok.kind == TokenType::TokenQuestion {
                let (bp, _) = self.binding_power(tok);
                if bp < min_bp {
                    break;
                }
                self.advance(); // ?
                let cond_after_left = self.parse_expression(0);
                if self.current_token().kind != TokenType::TokenColon {
                    panic!("Expected ':'") // TODO: proper error handling
                }
                self.advance();
                let cond_after_right = self.parse_expression(0);

                return Expr::Ternary(
                    Box::new(left),
                    Box::new(cond_after_left),
                    Box::new(cond_after_right),
                );
            }

            // function call
            if tok.kind == TokenType::TokenOpenParen {
                let bp = self.binding_power(tok).0;
                if bp < min_bp {
                    break;
                }
                self.advance();

                let mut args = vec![];
                while self.current_token().kind != TokenType::TokenCloseParen {
                    args.push(self.parse_expression(0));
                    while self.current_token().kind == TokenType::TokenComma {
                        self.advance();
                        args.push(self.parse_expression(0));
                    }
                }

                if self.current_token().kind != TokenType::TokenCloseParen {
                    panic!("Expected ')'"); // TODO: proper error handling
                }
                self.advance();

                left = Expr::FunctionCall(Box::new(left), args);
                continue;
            }

            // field access
            if tok.kind == TokenType::TokenDot || tok.kind == TokenType::TokenArrow {
                let bp = self.binding_power(tok).0;
                if bp < min_bp {
                    break;
                }
                self.advance(); // operator

                let field = match &self.current_token().kind {
                    TokenType::TokenIdentifier(sym) => *sym,
                    _ => panic!("Expected field name after '.'"),
                };
                self.advance(); // identifier
                left = Expr::FieldAccess(Box::new(left), field);

                continue;
            }

            // infix
            let (bp, right_assoc) = {
                if !tok.is_operator() {
                    break;
                }
                let (bp, right_assoc) = self.binding_power(tok);
                if bp < min_bp {
                    break;
                }
                (bp, right_assoc)
            };
            let op = BinaryOp::from_token(tok).unwrap();

            self.advance(); // consume the operator

            // Assignment operator should be right-assoc but subtraction cannot be right assoc so we
            // have to check for each operator
            // 1 + 2 + 3
            // RA: 1 + (2 + 3) = 6
            // LA: (1 + 2) + 3 = 6
            //
            // 1 - 2 - 3
            // RA: 1 - (2 - 3) = 2 This is false
            // LA: (1 - 2) - 3 = -4 This is true
            let right = self.parse_expression(bp + if right_assoc { 0 } else { 1 });

            left = Expr::Binary(Box::new(left), op, Box::new(right))
        }
        left
    }

    fn binding_power(&self, tok: &Token) -> (u8, bool) {
        match tok.kind {
            TokenType::TokenPlusAssign => (1, true),
            TokenType::TokenMinusAssign => (1, true),
            TokenType::TokenAsteriskAssign => (1, true),
            TokenType::TokenSlashAssign => (1, true),
            TokenType::TokenCarretAssign => (1, true),
            TokenType::TokenModuloAssign => (1, true),
            TokenType::TokenAmpersandAssign => (1, true),
            TokenType::TokenPipeAssign => (1, true),
            TokenType::TokenAssign => (1, true),
            TokenType::TokenQuestion => (4, false),
            TokenType::TokenDoublePipe => (5, false),
            TokenType::TokenDoubleAmpersand => (10, false),
            TokenType::TokenPipe => (15, false),
            TokenType::TokenCarret => (20, false),
            TokenType::TokenAmpersand => (25, false),
            TokenType::TokenEqual => (30, false),
            TokenType::TokenBangEqual => (30, false),
            TokenType::TokenOpenTag => (35, false),
            TokenType::TokenCloseTag => (35, false),
            TokenType::TokenLessEqual => (35, false),
            TokenType::TokenGreaterEqual => (35, false),
            TokenType::TokenBifShiftRight => (40, false),
            TokenType::TokenBifShiftLeft => (40, false),
            TokenType::TokenPlus => (50, false),
            TokenType::TokenMinus => (50, false),
            TokenType::TokenAsterisk => (60, false),
            TokenType::TokenSlash => (60, false),
            TokenType::TokenModulo => (60, false),
            TokenType::TokenOpenParen => (100, false),
            TokenType::TokenDot => (110, false),
            TokenType::TokenArrow => (110, false),

            _ => (0, false),
        }
    }
}

impl Token {
    pub fn is_operator(&self) -> bool {
        self.kind == TokenType::TokenPlus
            || self.kind == TokenType::TokenMinus
            || self.kind == TokenType::TokenSlash
            || self.kind == TokenType::TokenAsterisk
            || self.kind == TokenType::TokenPipe
            || self.kind == TokenType::TokenCarret
            || self.kind == TokenType::TokenAmpersand
            || self.kind == TokenType::TokenEqual
            || self.kind == TokenType::TokenBangEqual
            || self.kind == TokenType::TokenOpenTag
            || self.kind == TokenType::TokenCloseTag
            || self.kind == TokenType::TokenModulo
            || self.kind == TokenType::TokenBifShiftRight
            || self.kind == TokenType::TokenBifShiftLeft
            || self.kind == TokenType::TokenIncrement
            || self.kind == TokenType::TokenDecrement
            || self.kind == TokenType::TokenQuestion
            || self.kind == TokenType::TokenAssign
            || self.kind == TokenType::TokenDoubleAmpersand
            || self.kind == TokenType::TokenDoublePipe
            || self.kind == TokenType::TokenLessEqual
            || self.kind == TokenType::TokenGreaterEqual
            || self.kind == TokenType::TokenOpenParen
            || self.kind == TokenType::TokenDot
            || self.kind == TokenType::TokenArrow
            || self.kind == TokenType::TokenPlusAssign
            || self.kind == TokenType::TokenMinusAssign
            || self.kind == TokenType::TokenAsteriskAssign
            || self.kind == TokenType::TokenSlashAssign
            || self.kind == TokenType::TokenCarretAssign
            || self.kind == TokenType::TokenModuloAssign
            || self.kind == TokenType::TokenAmpersandAssign
            || self.kind == TokenType::TokenPipeAssign
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Interner;

    use super::*;
    #[test]
    fn test_basic_infix() {
        let mut interner = Interner::new();
        let input = "16<<2&&1*2-5";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_expression(0);

        //  ((16<<2)&&((1*2)-5))

        let expected_expression = Expr::Binary(
            Box::new(Expr::Binary(
                Box::new(Expr::Int(16)),
                BinaryOp::LeftShift,
                Box::new(Expr::Int(2)),
            )),
            BinaryOp::And,
            Box::new(Expr::Binary(
                Box::new(Expr::Binary(
                    Box::new(Expr::Int(1)),
                    BinaryOp::Mul,
                    Box::new(Expr::Int(2)),
                )),
                BinaryOp::Sub,
                Box::new(Expr::Int(5)),
            )),
        );

        assert_eq!(ast, expected_expression);
    }

    #[test]
    fn test_prefix_1() {
        let mut interner = Interner::new();
        let input = "1 * -2";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_expression(0);

        let expected_expression = Expr::Binary(
            Box::new(Expr::Int(1)),
            BinaryOp::Mul,
            Box::new(Expr::PrefixUnary(UnaryOp::Neg, Box::new(Expr::Int(2)))),
        );

        assert_eq!(ast, expected_expression);
    }

    #[test]
    fn test_prefix_2() {
        let mut interner = Interner::new();
        let input = "1 * --2";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_expression(0);

        let expected_expression = Expr::Binary(
            Box::new(Expr::Int(1)),
            BinaryOp::Mul,
            Box::new(Expr::PrefixUnary(
                UnaryOp::PreDecrement,
                Box::new(Expr::Int(2)),
            )),
        );

        assert_eq!(ast, expected_expression);
    }

    #[test]
    fn test_postfix() {
        let mut interner = Interner::new();
        let input = "1 * 2--";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_expression(0);

        let expected_expression = Expr::Binary(
            Box::new(Expr::Int(1)),
            BinaryOp::Mul,
            Box::new(Expr::PostfixUnary(
                Box::new(Expr::Int(2)),
                UnaryOp::PostDecrement,
            )),
        );

        assert_eq!(ast, expected_expression);
    }

    #[test]
    fn test_paren_precedence() {
        let mut interner = Interner::new();
        let input = "(1+2)*3";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_expression(0);

        let expected_expression = Expr::Binary(
            Box::new(Expr::Binary(
                Box::new(Expr::Int(1)),
                BinaryOp::Add,
                Box::new(Expr::Int(2)),
            )),
            BinaryOp::Mul,
            Box::new(Expr::Int(3)),
        );

        assert_eq!(ast, expected_expression);
    }

    #[test]
    fn test_ternary() {
        let mut interner = Interner::new();
        let input = "1+2 ? 2 : 3";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_expression(0);

        let expected_expression = Expr::Ternary(
            Box::new(Expr::Binary(
                Box::new(Expr::Int(1)),
                BinaryOp::Add,
                Box::new(Expr::Int(2)),
            )),
            Box::new(Expr::Int(2)),
            Box::new(Expr::Int(3)),
        );

        assert_eq!(ast, expected_expression);
    }

    #[test]
    fn test_identifier() {
        let mut interner = Interner::new();
        let input = "1 + foo * 2";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_expression(0);

        let expected_expression = Expr::Binary(
            Box::new(Expr::Int(1)),
            BinaryOp::Add,
            Box::new(Expr::Binary(
                Box::new(Expr::Ident(Symbol(0))),
                BinaryOp::Mul,
                Box::new(Expr::Int(2)),
            )),
        );

        assert_eq!(ast, expected_expression);
    }

    #[test]
    fn test_assignment() {
        let mut interner = Interner::new();
        let input = "1=2*3";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_expression(0);

        let expected_expression = Expr::Binary(
            Box::new(Expr::Int(1)),
            BinaryOp::Assign,
            Box::new(Expr::Binary(
                Box::new(Expr::Int(2)),
                BinaryOp::Mul,
                Box::new(Expr::Int(3)),
            )),
        );

        assert_eq!(ast, expected_expression);
    }

    #[test]
    fn test_multiple_assignments_right_assoc() {
        let mut interner = Interner::new();
        let input = "foo = bar = 2*3";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_expression(0);

        let expected_expression = Expr::Binary(
            Box::new(Expr::Ident(Symbol(0))),
            BinaryOp::Assign,
            Box::new(Expr::Binary(
                Box::new(Expr::Ident(Symbol(1))),
                BinaryOp::Assign,
                Box::new(Expr::Binary(
                    Box::new(Expr::Int(2)),
                    BinaryOp::Mul,
                    Box::new(Expr::Int(3)),
                )),
            )),
        );

        assert_eq!(ast, expected_expression);
    }

    #[test]
    fn test_block() {
        let mut interner = Interner::new();
        let input = "{foo=2*3}";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_expression = Statement::Block(vec![Statement::Expr(Expr::Binary(
            Box::new(Expr::Ident(Symbol(0))),
            BinaryOp::Assign,
            Box::new(Expr::Binary(
                Box::new(Expr::Int(2)),
                BinaryOp::Mul,
                Box::new(Expr::Int(3)),
            )),
        ))]);

        assert_eq!(ast, expected_expression);
    }

    #[test]
    fn test_if() {
        let mut interner = Interner::new();
        let input = "if (foo == bar){foo=2*3}";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_expression = Statement::If(
            Expr::Binary(
                Box::new(Expr::Ident(Symbol(0))),
                BinaryOp::Eq,
                Box::new(Expr::Ident(Symbol(1))),
            ),
            Box::new(Statement::Block(vec![Statement::Expr(Expr::Binary(
                Box::new(Expr::Ident(Symbol(0))),
                BinaryOp::Assign,
                Box::new(Expr::Binary(
                    Box::new(Expr::Int(2)),
                    BinaryOp::Mul,
                    Box::new(Expr::Int(3)),
                )),
            ))])),
            None,
        );

        assert_eq!(ast, expected_expression);
    }

    #[test]
    fn test_if_else() {
        let mut interner = Interner::new();
        let input = "if (foo == bar){foo=2*3} else {bar=2*3}";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_expression = Statement::If(
            Expr::Binary(
                Box::new(Expr::Ident(Symbol(0))),
                BinaryOp::Eq,
                Box::new(Expr::Ident(Symbol(1))),
            ),
            Box::new(Statement::Block(vec![Statement::Expr(Expr::Binary(
                Box::new(Expr::Ident(Symbol(0))),
                BinaryOp::Assign,
                Box::new(Expr::Binary(
                    Box::new(Expr::Int(2)),
                    BinaryOp::Mul,
                    Box::new(Expr::Int(3)),
                )),
            ))])),
            Some(Box::new(Statement::Block(vec![Statement::Expr(
                Expr::Binary(
                    Box::new(Expr::Ident(Symbol(1))),
                    BinaryOp::Assign,
                    Box::new(Expr::Binary(
                        Box::new(Expr::Int(2)),
                        BinaryOp::Mul,
                        Box::new(Expr::Int(3)),
                    )),
                ),
            )]))),
        );

        assert_eq!(ast, expected_expression);
    }

    #[test]
    fn test_if_elseif() {
        let mut interner = Interner::new();
        let input = "if (foo == bar){foo=2*3} else if (foo==bar){bar=2*3}";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_expression = Statement::If(
            Expr::Binary(
                Box::new(Expr::Ident(Symbol(0))),
                BinaryOp::Eq,
                Box::new(Expr::Ident(Symbol(1))),
            ),
            Box::new(Statement::Block(vec![Statement::Expr(Expr::Binary(
                Box::new(Expr::Ident(Symbol(0))),
                BinaryOp::Assign,
                Box::new(Expr::Binary(
                    Box::new(Expr::Int(2)),
                    BinaryOp::Mul,
                    Box::new(Expr::Int(3)),
                )),
            ))])),
            Some(Box::new(Statement::If(
                Expr::Binary(
                    Box::new(Expr::Ident(Symbol(0))),
                    BinaryOp::Eq,
                    Box::new(Expr::Ident(Symbol(1))),
                ),
                Box::new(Statement::Block(vec![Statement::Expr(Expr::Binary(
                    Box::new(Expr::Ident(Symbol(1))),
                    BinaryOp::Assign,
                    Box::new(Expr::Binary(
                        Box::new(Expr::Int(2)),
                        BinaryOp::Mul,
                        Box::new(Expr::Int(3)),
                    )),
                ))])),
                None,
            ))),
        );

        assert_eq!(ast, expected_expression);
    }

    #[test]
    fn test_while() {
        let mut interner = Interner::new();
        let input = "while(foo==bar){foo=2*3}";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_expression = Statement::While(
            Expr::Binary(
                Box::new(Expr::Ident(Symbol(0))),
                BinaryOp::Eq,
                Box::new(Expr::Ident(Symbol(1))),
            ),
            Box::new(Statement::Block(vec![Statement::Expr(Expr::Binary(
                Box::new(Expr::Ident(Symbol(0))),
                BinaryOp::Assign,
                Box::new(Expr::Binary(
                    Box::new(Expr::Int(2)),
                    BinaryOp::Mul,
                    Box::new(Expr::Int(3)),
                )),
            ))])),
        );

        assert_eq!(ast, expected_expression);
    }

    #[test]
    fn test_do_while() {
        let mut interner = Interner::new();
        let input = "do {foo=2*3} while(foo==bar)";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_expression = Statement::DoWhile(
            Box::new(Statement::Block(vec![Statement::Expr(Expr::Binary(
                Box::new(Expr::Ident(Symbol(0))),
                BinaryOp::Assign,
                Box::new(Expr::Binary(
                    Box::new(Expr::Int(2)),
                    BinaryOp::Mul,
                    Box::new(Expr::Int(3)),
                )),
            ))])),
            Expr::Binary(
                Box::new(Expr::Ident(Symbol(0))),
                BinaryOp::Eq,
                Box::new(Expr::Ident(Symbol(1))),
            ),
        );

        assert_eq!(ast, expected_expression);
    }

    #[test]
    fn test_return() {
        let mut interner = Interner::new();
        let input = "return foo+1";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_expression = Statement::Return(Some(Expr::Binary(
            Box::new(Expr::Ident(Symbol(0))),
            BinaryOp::Add,
            Box::new(Expr::Int(1)),
        )));

        assert_eq!(ast, expected_expression);
    }

    #[test]
    fn test_function_call() {
        let mut interner = Interner::new();
        let input = "bar = foo(1,2)";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_expression(0);

        let expected_expression = Expr::Binary(
            Box::new(Expr::Ident(Symbol(0))),
            BinaryOp::Assign,
            Box::new(Expr::FunctionCall(
                Box::new(Expr::Ident(Symbol(1))),
                vec![Expr::Int(1), Expr::Int(2)],
            )),
        );

        assert_eq!(ast, expected_expression);
    }

    #[test]
    fn test_field_access_1() {
        let mut interner = Interner::new();
        let input = "foo.bar = 1";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_expression(0);

        let expected_expression = Expr::Binary(
            Box::new(Expr::FieldAccess(
                Box::new(Expr::Ident(Symbol(0))),
                Symbol(1),
            )),
            BinaryOp::Assign,
            Box::new(Expr::Int(1)),
        );

        assert_eq!(ast, expected_expression);
    }

    #[test]
    fn test_field_access_2() {
        let mut interner = Interner::new();
        let input = "foo->bar = 1";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_expression(0);

        let expected_expression = Expr::Binary(
            Box::new(Expr::FieldAccess(
                Box::new(Expr::Ident(Symbol(0))),
                Symbol(1),
            )),
            BinaryOp::Assign,
            Box::new(Expr::Int(1)),
        );

        assert_eq!(ast, expected_expression);
    }

    #[test]
    fn test_compound_assignment() {
        let mut interner = Interner::new();
        let input = "foo*=2%2";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_expression(0);

        let expected_expression = Expr::Binary(
            Box::new(Expr::Ident(Symbol(0))),
            BinaryOp::MulAssign,
            Box::new(Expr::Binary(
                Box::new(Expr::Int(2)),
                BinaryOp::Modulo,
                Box::new(Expr::Int(2)),
            )),
        );

        assert_eq!(ast, expected_expression);
    }
}
