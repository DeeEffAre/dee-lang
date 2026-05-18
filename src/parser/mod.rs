use crate::lexer::{CompileError, Lexer, Symbol, Token, TokenType};

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
    ArraySub(Box<Expr>, Box<Expr>),
    Match(Box<Expr>, Vec<(Pattern, Statement)>),
    Variant(Box<Expr>, Pattern),

    // builtin variants
    None,
    Ok(Box<Expr>),
    Some(Box<Expr>),
    Err(Box<Expr>),

    ArrayInit(Vec<Expr>),

    Error,
}

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Wildcard,
    None,
    Some(Box<Pattern>),
    Ok(Box<Pattern>),
    Err(Box<Pattern>),
    Variant(Symbol, Box<Pattern>),
    Binding(Symbol),
}

#[derive(Debug, PartialEq)]
pub struct FunctionArg {
    pub ty: Type,
    pub name: Symbol,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Block(Vec<Statement>),
    Expr(Expr),
    If(Expr, Box<Statement>, Option<Box<Statement>>),
    While(Expr, Box<Statement>),
    DoWhile(Box<Statement>, Expr),
    Return(Option<Expr>),
    Declaration(Type, Symbol, Expr),
    ShortDeclaration(Type, Symbol),

    // i64 sum(i32 num1, i32 num2) -> Calculator {};
    FunctionDeclaration(
        bool,
        Type,
        Symbol,
        Vec<FunctionArg>,
        Option<Symbol>,
        Box<Statement>,
    ),

    // struct Calculator {type: field}
    Struct(Symbol, Vec<(Type, Symbol)>),
    Defer(Box<Statement>),
    For(Box<Statement>, Expr, Expr, Box<Statement>),

    Break,
    Continue,

    Error,
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
    AddressOf,
    Dereference,
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

#[derive(Debug, PartialEq)]
pub enum Type {
    I64,
    I32,
    I16,
    I8,
    U64,
    U32,
    U16,
    U8,
    Usize,
    F64,
    F32,
    String,
    Bool,
    Char,
    DynArr,
    FixArray(Box<Type>),
    Hashmap,
    Variant,
    Struct,
    Result,
    Maybe,
}

impl Type {
    fn from_token(token: &Token) -> Option<Self> {
        match token.kind {
            TokenType::TokenI64 => Some(Type::I64),
            TokenType::TokenI32 => Some(Type::I32),
            TokenType::TokenI16 => Some(Type::I16),
            TokenType::TokenI8 => Some(Type::I8),
            TokenType::TokenU64 => Some(Type::U64),
            TokenType::TokenU32 => Some(Type::U32),
            TokenType::TokenU16 => Some(Type::U16),
            TokenType::TokenU8 => Some(Type::U8),
            TokenType::TokenF64 => Some(Type::F64),
            TokenType::TokenF32 => Some(Type::F32),
            TokenType::TokenUsize => Some(Type::Usize),
            TokenType::TokenBoolean => Some(Type::Bool),
            TokenType::TokenVariant => Some(Type::Variant),
            TokenType::TokenMaybe => Some(Type::Maybe),
            TokenType::TokenResult => Some(Type::Result),
            TokenType::TokenArray => Some(Type::DynArr),
            TokenType::TokenHashmap => Some(Type::Hashmap),
            TokenType::TokenChar => Some(Type::Char),
            TokenType::TokenStruct => Some(Type::Struct),
            TokenType::TokenString => Some(Type::String),
            _ => None,
        }
    }
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
    peeked_token: Token,
    errors: Vec<CompileError>,
}

impl<'a> Parser<'a> {
    #[must_use]
    pub fn init_parser(lexer: &'a mut Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::default(),
            peeked_token: Token::default(),
            errors: Vec::new(),
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

    fn synchronize(&mut self) {
        while self.current_token().kind != TokenType::TokenSemicolon
            && self.current_token().kind != TokenType::TokenCloseBrace
            && self.current_token().kind != TokenType::TokenEOF
        {
            self.advance();
        }
        if self.current_token().kind == TokenType::TokenSemicolon {
            self.advance();
        }
    }

    pub fn parse_statement(&mut self) -> Statement {
        if self.current_token().is_type() {
            return self.parse_declaration();
        }

        match self.current_token().kind {
            TokenType::TokenOpenBrace => self.parse_block(),
            TokenType::TokenIf => self.parse_if(),
            TokenType::TokenWhile => self.parse_while().expect("Expected while block"),
            TokenType::TokenDo => self.parse_do_while().expect("Expected return statement"),
            TokenType::TokenReturn => self.parse_return().expect("Expected return statement"),
            TokenType::TokenFunc | TokenType::TokenStatic => self
                .parse_function_declaration()
                .expect("Expected a function declaration"),
            TokenType::TokenStruct => self.parse_struct_definition().expect("Expected a struct"),
            TokenType::TokenDefer => self.parse_defer().expect("Expected a defer statement"),
            TokenType::TokenFor => self.parse_for().expect("Expected a for loop"),
            TokenType::TokenContinue => {
                self.advance(); // continue
                self.advance(); // ;
                Statement::Continue
            }
            TokenType::TokenBreak => {
                self.advance(); // break
                self.advance(); // ;
                Statement::Break
            }
            _ => {
                let expr = self.parse_expression(0);
                if matches!(self.current_token().kind, TokenType::TokenSemicolon) {
                    self.advance();
                }
                Statement::Expr(expr)
            }
        }
    }

    fn parse_type(&mut self) -> Type {
        let base = Type::from_token(self.current_token()).expect("Expected type");
        self.advance();

        // FixArray check
        if self.current_token().kind == TokenType::TokenOpenSquare {
            self.advance(); // [
            if self.current_token().kind != TokenType::TokenCloseSquare {
                panic!("Expected ']' in array type");
            }
            self.advance(); // ]
            return Type::FixArray(Box::new(base));
        }

        base
    }

    fn parse_declaration(&mut self) -> Statement {
        let var_type = self.parse_type();

        let variable = match &self.current_token().kind {
            TokenType::TokenIdentifier(sym) => *sym,
            _ => {
                self.errors.push(CompileError {
                    message: "Expected a variable name".into(),
                    span: self.current_token().span,
                });
                self.synchronize();
                return Statement::Error;
            }
        };
        self.advance();

        if self.current_token().kind != TokenType::TokenAssign {
            if self.current_token().kind != TokenType::TokenSemicolon {
                self.errors.push(CompileError {
                    message: "Expected '=' or ';'".into(),
                    span: self.current_token().span,
                });
                self.synchronize();
                return Statement::Error;
            }
            self.advance(); // ;

            return Statement::ShortDeclaration(var_type, variable);
        }
        self.advance(); // =

        let variable_value = self.parse_expression(0);

        if self.current_token().kind != TokenType::TokenSemicolon {
            self.errors.push(CompileError {
                message: "Missing semicolon at the end of the statement".into(),
                span: self.current_token().span,
            });
            return Statement::Error;
        }
        self.advance(); // ;

        Statement::Declaration(var_type, variable, variable_value)
    }

    fn parse_function_declaration_arg(&mut self) -> (Type, Symbol) {
        let arg_type = self.parse_type();
        let arg_symbol = match &self.current_token().kind {
            TokenType::TokenIdentifier(sym) => *sym,
            _ => panic!("Expected argument name"), // TODO: proper error handling
        };
        self.advance();

        (arg_type, arg_symbol)
    }

    fn parse_function_declaration(&mut self) -> Option<Statement> {
        let mut static_modifier = false;
        if self.current_token().kind == TokenType::TokenStatic {
            static_modifier = true;
            self.advance(); // static
        }

        self.advance(); // func

        let return_type = self.parse_type();

        let function_name = match &self.current_token().kind {
            TokenType::TokenIdentifier(sym) => *sym,
            _ => panic!("Expected function name"),
        };
        self.advance();

        if self.current_token().kind != TokenType::TokenOpenParen {
            panic!("Expected '('") // TODO: proper error handling
        }
        self.advance(); // (

        let mut args = vec![];
        while self.current_token().kind != TokenType::TokenCloseParen {
            let (arg_type, arg_symbol) = self.parse_function_declaration_arg();

            args.push(FunctionArg {
                ty: arg_type,
                name: arg_symbol,
            });

            while self.current_token().kind == TokenType::TokenComma {
                self.advance();
                let (arg_type, arg_symbol) = self.parse_function_declaration_arg();
                args.push(FunctionArg {
                    ty: arg_type,
                    name: arg_symbol,
                });
            }
        }

        if self.current_token().kind != TokenType::TokenCloseParen {
            panic!("Expected ')'"); // TODO: proper error handling
        }
        self.advance();

        // Method binding
        let binded_struct = if self.current_token().kind == TokenType::TokenArrow {
            self.advance(); // ->

            let symbol = match &self.current_token().kind {
                TokenType::TokenIdentifier(sym) => *sym,
                _ => panic!("Expected function name"),
            };
            self.advance();

            Some(symbol)
        } else {
            None
        };

        let function_block = self.parse_statement();

        Some(Statement::FunctionDeclaration(
            static_modifier,
            return_type,
            function_name,
            args,
            binded_struct,
            Box::new(function_block),
        ))
    }

    fn parse_struct_definition(&mut self) -> Option<Statement> {
        self.advance(); // struct

        let struct_name = match &self.current_token().kind {
            TokenType::TokenIdentifier(sym) => *sym,
            _ => panic!("Expected struct name"), // TODO: proper error handling
        };
        self.advance();

        if self.current_token().kind != TokenType::TokenOpenBrace {
            panic!("Expected '{{'") // TODO: proper error handling
        }
        self.advance(); // {

        let mut type_field_pairs = vec![];
        while self.current_token().kind != TokenType::TokenCloseBrace
            && self.current_token().kind != TokenType::TokenEOF
        {
            let field_type = self.parse_type();
            let field_name = match &self.current_token().kind {
                TokenType::TokenIdentifier(sym) => *sym,
                _ => panic!("Expected field name"), // TODO: proper error handling
            };
            self.advance();

            if self.current_token().kind != TokenType::TokenSemicolon {
                panic!("Expected ';'") // TODO: proper error handling
            }
            self.advance();

            type_field_pairs.push((field_type, field_name));
        }

        if self.current_token().kind == TokenType::TokenEOF {
            panic!("Expected '}}'") // TODO: proper error handling
        }

        self.advance(); // }

        Some(Statement::Struct(struct_name, type_field_pairs))
    }

    fn parse_pattern(&mut self) -> Pattern {
        match &self.current_token().kind {
            TokenType::TokenUnderscore => {
                self.advance();
                Pattern::Wildcard
            }
            TokenType::TokenNone => {
                self.advance();
                Pattern::None
            }
            TokenType::TokenSome => {
                self.advance();
                if self.current_token().kind != TokenType::TokenOpenParen {
                    panic!("Expected '(' after Some");
                }
                self.advance(); // (
                let inner = self.parse_pattern();
                if self.current_token().kind != TokenType::TokenCloseParen {
                    panic!("Expected ')' after Some pattern");
                }
                self.advance(); // )
                Pattern::Some(Box::new(inner))
            }
            TokenType::TokenOk => {
                self.advance();
                if self.current_token().kind != TokenType::TokenOpenParen {
                    panic!("Expected '(' after Some");
                }
                self.advance(); // (
                let inner = self.parse_pattern();
                if self.current_token().kind != TokenType::TokenCloseParen {
                    panic!("Expected ')' after Some pattern");
                }
                self.advance(); // )
                Pattern::Ok(Box::new(inner))
            }
            TokenType::TokenErr => {
                self.advance();
                if self.current_token().kind != TokenType::TokenOpenParen {
                    panic!("Expected '(' after Some");
                }
                self.advance(); // (
                let inner = self.parse_pattern();
                if self.current_token().kind != TokenType::TokenCloseParen {
                    panic!("Expected ')' after Some pattern");
                }
                self.advance(); // )
                Pattern::Err(Box::new(inner))
            }
            TokenType::TokenIdentifier(sym) => {
                let sym = *sym;
                self.advance();
                if self.current_token().kind == TokenType::TokenOpenParen {
                    self.advance(); // (
                    let inner = self.parse_pattern();
                    if self.current_token().kind != TokenType::TokenCloseParen {
                        panic!("Expected ')' after variant pattern");
                    }
                    self.advance(); // )
                    Pattern::Variant(sym, Box::new(inner))
                } else {
                    Pattern::Binding(sym)
                }
            }
            _ => panic!("Expected pattern"),
        }
    }

    fn parse_match(&mut self) -> Option<Expr> {
        self.advance(); // match

        if self.current_token().kind != TokenType::TokenOpenParen {
            panic!("Expected '('"); // TODO: proper error handling
        }
        self.advance(); // (

        let eval_expr = self.parse_expression(0);

        if self.current_token().kind != TokenType::TokenCloseParen {
            panic!("Expected ')'"); // TODO: proper error handling
        }
        self.advance(); // )

        if self.current_token().kind != TokenType::TokenOpenBrace {
            panic!("Expected '{{'"); // TODO: proper error handling
        }
        self.advance(); // {

        let mut arms = vec![];
        while self.current_token().kind != TokenType::TokenCloseBrace
            && self.current_token().kind != TokenType::TokenEOF
        {
            if self.current_token().kind != TokenType::TokenPipe {
                panic!("Expected '|'"); // TODO: proper error handling
            }
            self.advance(); // |

            let pattern = self.parse_pattern();

            if self.current_token.kind != TokenType::TokenArrow {
                panic!("Expected '->'"); // TODO: proper error handling
            }
            self.advance(); // ->

            let arm_statement = self.parse_statement();

            arms.push((pattern, arm_statement));
        }

        if self.current_token().kind != TokenType::TokenCloseBrace {
            panic!("Expected '}}'"); // TODO: proper error handling
        }
        self.advance();

        Some(Expr::Match(Box::new(eval_expr), arms))
    }

    fn parse_defer(&mut self) -> Option<Statement> {
        self.advance(); // defer

        let defer_statement = self.parse_statement();

        Some(Statement::Defer(Box::new(defer_statement)))
    }

    fn parse_for(&mut self) -> Option<Statement> {
        self.advance(); // for

        if self.current_token().kind != TokenType::TokenOpenParen {
            panic!("Expected '('"); // TODO: proper error handling
        }
        self.advance(); // (

        let for_declaration = self.parse_statement();

        let for_condition = self.parse_expression(0);

        if self.current_token().kind != TokenType::TokenSemicolon {
            panic!("Expected ';'"); // TODO: proper error handling
        }
        self.advance(); // ;

        let for_step = self.parse_expression(0);

        if self.current_token().kind != TokenType::TokenCloseParen {
            panic!("Expected ')'"); // TODO: proper error handling
        }
        self.advance(); // )

        let for_block = self.parse_statement();

        Some(Statement::For(
            Box::new(for_declaration),
            for_condition,
            for_step,
            Box::new(for_block),
        ))
    }

    fn parse_block(&mut self) -> Statement {
        self.advance(); // {

        let mut statements = vec![];
        while self.current_token().kind != TokenType::TokenCloseBrace
            && self.current_token().kind != TokenType::TokenEOF
        {
            statements.push(self.parse_statement());
        }

        if self.current_token().kind == TokenType::TokenEOF {
            self.errors.push(CompileError {
                message: "Expected '}'".into(),
                span: self.current_token().span,
            });
            return Statement::Error;
        }

        self.advance(); // }
        Statement::Block(statements)
    }

    fn parse_if(&mut self) -> Statement {
        self.advance(); // if 

        if self.current_token().kind != TokenType::TokenOpenParen {
            self.errors.push(CompileError {
                message: "Expected '('".into(),
                span: self.current_token().span,
            });
            self.synchronize();
            return Statement::Error;
        }
        self.advance(); // (

        let cond = self.parse_expression(0);

        if self.current_token().kind != TokenType::TokenCloseParen {
            self.errors.push(CompileError {
                message: "Expected ')'".into(),
                span: self.current_token().span,
            });
            self.synchronize();
            return Statement::Error;
        }

        self.advance(); // )

        if self.current_token().kind != TokenType::TokenOpenBrace {
            self.errors.push(CompileError {
                message: "Expected '{'".into(),
                span: self.current_token().span,
            });
            self.synchronize();
            return Statement::Error;
        }

        let if_block = self.parse_statement();

        if self.current_token().kind == TokenType::TokenElse {
            self.advance(); // else
            let else_branch = if self.current_token().kind == TokenType::TokenIf {
                self.parse_if()
            } else {
                self.parse_statement()
            };
            Statement::If(cond, Box::new(if_block), Some(Box::new(else_branch)))
        } else {
            Statement::If(cond, Box::new(if_block), None)
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

        if self.current_token().kind != TokenType::TokenSemicolon {
            panic!("Missing semicolon at the end of the statement"); // TODO: proper error handling
        }
        self.advance(); // ;

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

            // builtin variants
            TokenType::TokenNone => {
                self.advance();
                return Some(Expr::None);
            }
            TokenType::TokenOk => {
                self.advance();

                if self.current_token().kind != TokenType::TokenOpenParen {
                    panic!("Expected '('"); // TODO: proper error handling
                }
                let inner = self.parse_primary().expect("Expected value");

                return Some(Expr::Ok(Box::new(inner)));
            }
            TokenType::TokenSome => {
                self.advance();

                if self.current_token().kind != TokenType::TokenOpenParen {
                    panic!("Expected '('"); // TODO: proper error handling
                }
                let inner = self.parse_primary().expect("Expected value");

                return Some(Expr::Some(Box::new(inner)));
            }
            TokenType::TokenErr => {
                self.advance();

                if self.current_token().kind != TokenType::TokenOpenParen {
                    panic!("Expected '('"); // TODO: proper error handling
                }
                let inner = self.parse_primary().expect("Expected value");

                return Some(Expr::Err(Box::new(inner)));
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
            TokenType::TokenAmpersand => {
                self.advance();
                let right = self.parse_expression(70);
                Expr::PrefixUnary(UnaryOp::AddressOf, Box::new(right))
            }
            TokenType::TokenAsterisk => {
                self.advance();
                let right = self.parse_expression(70);
                Expr::PrefixUnary(UnaryOp::Dereference, Box::new(right))
            }
            TokenType::TokenMatch => return self.parse_match().expect("Expected match"),
            TokenType::TokenOpenBrace => {
                self.advance(); // {

                let mut elems = vec![];
                while self.current_token().kind != TokenType::TokenCloseBrace {
                    elems.push(self.parse_expression(0));
                    while self.current_token().kind == TokenType::TokenComma {
                        self.advance();
                        elems.push(self.parse_expression(0));
                    }
                }

                if self.current_token().kind != TokenType::TokenCloseBrace {
                    panic!("Expected '}}'"); // TODO: proper error handling
                }
                self.advance();

                Expr::ArrayInit(elems)
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

            // array subscript
            if tok.kind == TokenType::TokenOpenSquare {
                let bp = self.binding_power(tok).0;
                if bp < min_bp {
                    break;
                }
                self.advance();

                if self.current_token().kind == TokenType::TokenCloseSquare {
                    panic!("Empty subscript")
                }

                let sub = self.parse_expression(0); // inside of []

                if self.current_token().kind != TokenType::TokenCloseSquare {
                    panic!("Expected ']'"); // TODO: proper error handling
                }
                self.advance();

                left = Expr::ArraySub(Box::new(left), Box::new(sub));
                continue;
            }

            // variant (enum) initialization
            if tok.kind == TokenType::TokenColon
                && self.peeked_token().kind == TokenType::TokenColon
            {
                self.advance(); // :

                if self.current_token().kind != TokenType::TokenColon {
                    panic!("Expected ':'"); // TODO: proper error handling
                }
                self.advance(); // :

                let pattern = self.parse_pattern();

                left = Expr::Variant(Box::new(left), pattern);

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

    fn is_type(&self) -> bool {
        self.kind == TokenType::TokenI64
            || self.kind == TokenType::TokenI32
            || self.kind == TokenType::TokenI16
            || self.kind == TokenType::TokenI8
            || self.kind == TokenType::TokenU64
            || self.kind == TokenType::TokenU32
            || self.kind == TokenType::TokenU16
            || self.kind == TokenType::TokenU8
            || self.kind == TokenType::TokenF64
            || self.kind == TokenType::TokenF32
            || self.kind == TokenType::TokenBoolean
            || self.kind == TokenType::TokenVariant
            || self.kind == TokenType::TokenMaybe
            || self.kind == TokenType::TokenResult
            || self.kind == TokenType::TokenArray
            || self.kind == TokenType::TokenHashmap
            || self.kind == TokenType::TokenChar
            || self.kind == TokenType::TokenString
            || self.kind == TokenType::TokenUsize
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
    fn test_prefix_3() {
        let mut interner = Interner::new();
        let input = "&foo + *bar";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_expression(0);

        let expected_expression = Expr::Binary(
            Box::new(Expr::PrefixUnary(
                UnaryOp::AddressOf,
                Box::new(Expr::Ident(Symbol(0))),
            )),
            BinaryOp::Add,
            Box::new(Expr::PrefixUnary(
                UnaryOp::Dereference,
                Box::new(Expr::Ident(Symbol(1))),
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

    // BLOCK
    #[test]
    fn test_block() {
        let mut interner = Interner::new();
        let input = "{foo=2*3;}";
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
    fn test_block_missing_closing_brace() {
        let mut interner = Interner::new();
        let input = "{foo=2*3;";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let err = parser.errors.first().unwrap().message.clone();

        assert_eq!(ast, Statement::Error);
        assert_eq!(err, "Expected '}'".into());
    }

    #[test]
    fn test_block_multiline() {
        let mut interner = Interner::new();
        let input = "{foo=2*3;i8 bar = 1+1;}";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_expression = Statement::Block(vec![
            Statement::Expr(Expr::Binary(
                Box::new(Expr::Ident(Symbol(0))),
                BinaryOp::Assign,
                Box::new(Expr::Binary(
                    Box::new(Expr::Int(2)),
                    BinaryOp::Mul,
                    Box::new(Expr::Int(3)),
                )),
            )),
            Statement::Declaration(
                Type::I8,
                Symbol(1),
                Expr::Binary(
                    Box::new(Expr::Int(1)),
                    BinaryOp::Add,
                    Box::new(Expr::Int(1)),
                ),
            ),
        ]);

        assert_eq!(ast, expected_expression);
    }

    // IF
    #[test]
    fn test_if() {
        let mut interner = Interner::new();
        let input = "if (foo == bar){foo=2*3;}";
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
    fn test_if_missing_open_paren() {
        let mut interner = Interner::new();
        let input = "if foo == bar){foo=2*3;}";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let err = parser.errors.first().unwrap().message.clone();

        assert_eq!(ast, Statement::Error);
        assert_eq!(err, "Expected '('".into())
    }

    #[test]
    fn test_if_missing_close_paren() {
        let mut interner = Interner::new();
        let input = "if (foo == bar{foo=2*3;}";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let err = parser.errors.first().unwrap().message.clone();

        assert_eq!(ast, Statement::Error);
        assert_eq!(err, "Expected ')'".into())
    }

    #[test]
    fn test_if_missing_open_brace() {
        let mut interner = Interner::new();
        let input = "if (foo == bar)foo=2*3;}";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let err = parser.errors.first().unwrap().message.clone();

        assert_eq!(ast, Statement::Error);
        assert_eq!(err, "Expected '{'".into())
    }

    #[test]
    fn test_if_else() {
        let mut interner = Interner::new();
        let input = "if (foo == bar){foo=2*3;} else {bar=2*3;}";
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
        let input = "if (foo == bar){foo=2*3;} else if (foo==bar){bar=2*3;}";
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
        let input = "while(foo==bar){foo=2*3;}";
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
        let input = "do {foo=2*3;} while(foo==bar)";
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
        let input = "return foo+1;";
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
    fn test_return_multiline() {
        let mut interner = Interner::new();
        let input = "return foo+1;return 1;";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast1 = parser.parse_statement();

        let expected_expression1 = Statement::Return(Some(Expr::Binary(
            Box::new(Expr::Ident(Symbol(0))),
            BinaryOp::Add,
            Box::new(Expr::Int(1)),
        )));

        let ast2 = parser.parse_statement();

        let expected_expression2 = Statement::Return(Some(Expr::Int(1)));

        assert_eq!(ast1, expected_expression1);
        assert_eq!(ast2, expected_expression2);
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

    #[test]
    fn test_array_sub() {
        let mut interner = Interner::new();
        let input = "foo[1*2+1]";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_expression(0);

        let expected_expression = Expr::ArraySub(
            Box::new(Expr::Ident(Symbol(0))),
            Box::new(Expr::Binary(
                Box::new(Expr::Binary(
                    Box::new(Expr::Int(1)),
                    BinaryOp::Mul,
                    Box::new(Expr::Int(2)),
                )),
                BinaryOp::Add,
                Box::new(Expr::Int(1)),
            )),
        );

        assert_eq!(ast, expected_expression);
    }

    #[test]
    #[should_panic(expected = "Empty subscript")]
    fn test_array_sub_empty_expression() {
        let mut interner = Interner::new();
        let input = "bar = foo[]";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let _ast = parser.parse_expression(0);
    }

    // DECLARATION
    #[test]
    fn test_declaration() {
        let mut interner = Interner::new();
        let input = "i64 bar = 132+123;";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_statement = Statement::Declaration(
            Type::I64,
            Symbol(0),
            Expr::Binary(
                Box::new(Expr::Int(132)),
                BinaryOp::Add,
                Box::new(Expr::Int(123)),
            ),
        );

        assert_eq!(ast, expected_statement);
    }

    #[test]
    fn test_short_declaration() {
        let mut interner = Interner::new();
        let input = "i64 foo;";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_statement = Statement::ShortDeclaration(Type::I64, Symbol(0));

        assert_eq!(ast, expected_statement);
    }

    #[test]
    fn test_short_declaration_missing_semicolon() {
        let mut interner = Interner::new();
        let input = "i64 foo";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let err = parser.errors.first().unwrap().message.clone();

        assert_eq!(ast, Statement::Error);
        assert_eq!(err, "Expected '=' or ';'".into())
    }

    #[test]
    fn test_declaration_missing_semicolon() {
        let mut interner = Interner::new();
        let input = "i64 bar = 132+123";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();
        assert_eq!(ast, Statement::Error);

        let err = parser.errors.first().unwrap().message.clone();
        assert_eq!(err, "Missing semicolon at the end of the statement".into())
    }

    #[test]
    fn test_declaration_missing_assign_token() {
        let mut interner = Interner::new();
        let input = "i64 bar  132+123";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();
        assert_eq!(ast, Statement::Error);

        let err = parser.errors.first().unwrap().message.clone();
        assert_eq!(err, "Expected '=' or ';'".into())
    }

    #[test]
    fn test_declaration_missing_variable_name() {
        let mut interner = Interner::new();
        let input = "i64 = 132+123;i64 bar = 132+123;";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast1 = parser.parse_statement();
        let ast2 = parser.parse_statement();

        let expected_statement2 = Statement::Declaration(
            Type::I64,
            Symbol(0),
            Expr::Binary(
                Box::new(Expr::Int(132)),
                BinaryOp::Add,
                Box::new(Expr::Int(123)),
            ),
        );

        let err1 = parser.errors.first().unwrap().message.clone();

        assert_eq!(ast1, Statement::Error);
        assert_eq!(err1, "Expected a variable name".into());
        assert_eq!(ast2, expected_statement2);
    }

    #[test]
    fn test_declaration_fixed_array() {
        let mut interner = Interner::new();
        // WARN: change the expression of this test case after adding parser for array initialization
        let input = "i64[] bar = 132+123;";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_statement = Statement::Declaration(
            Type::FixArray(Box::new(Type::I64)),
            Symbol(0),
            Expr::Binary(
                Box::new(Expr::Int(132)),
                BinaryOp::Add,
                Box::new(Expr::Int(123)),
            ),
        );

        assert_eq!(ast, expected_statement);
    }

    #[test]
    fn test_function_declaration_without_method_binding() {
        let mut interner = Interner::new();
        let input = "func i64 sum(i64 num1, i64 num2) {return num1 + num2;}";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_statement = Statement::FunctionDeclaration(
            false,
            Type::I64,
            Symbol(0),
            vec![
                FunctionArg {
                    ty: Type::I64,
                    name: Symbol(1),
                },
                FunctionArg {
                    ty: Type::I64,
                    name: Symbol(2),
                },
            ],
            None,
            Box::new(Statement::Block(vec![Statement::Return(Some(
                Expr::Binary(
                    Box::new(Expr::Ident(Symbol(1))),
                    BinaryOp::Add,
                    Box::new(Expr::Ident(Symbol(2))),
                ),
            ))])),
        );

        assert_eq!(ast, expected_statement);
    }

    #[test]
    fn test_function_declaration_without_args_and_without_method_binding() {
        let mut interner = Interner::new();
        let input = "func i64 sum() {return num1 + num2;}";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_statement = Statement::FunctionDeclaration(
            false,
            Type::I64,
            Symbol(0),
            vec![],
            None,
            Box::new(Statement::Block(vec![Statement::Return(Some(
                Expr::Binary(
                    Box::new(Expr::Ident(Symbol(1))),
                    BinaryOp::Add,
                    Box::new(Expr::Ident(Symbol(2))),
                ),
            ))])),
        );

        assert_eq!(ast, expected_statement);
    }

    #[test]
    fn test_function_declaration_with_method_binding() {
        let mut interner = Interner::new();
        let input = "func i64 sum(i64 num1, i64 num2) -> Parser {return num1 + num2;}";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_statement = Statement::FunctionDeclaration(
            false,
            Type::I64,
            Symbol(0),
            vec![
                FunctionArg {
                    ty: Type::I64,
                    name: Symbol(1),
                },
                FunctionArg {
                    ty: Type::I64,
                    name: Symbol(2),
                },
            ],
            Some(Symbol(3)),
            Box::new(Statement::Block(vec![Statement::Return(Some(
                Expr::Binary(
                    Box::new(Expr::Ident(Symbol(1))),
                    BinaryOp::Add,
                    Box::new(Expr::Ident(Symbol(2))),
                ),
            ))])),
        );

        assert_eq!(ast, expected_statement);
    }

    #[test]
    fn test_function_declaration_without_args_and_with_method_binding() {
        let mut interner = Interner::new();
        let input = "func i64 sum() -> Parser {return 1 + 1;}";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_statement = Statement::FunctionDeclaration(
            false,
            Type::I64,
            Symbol(0),
            vec![],
            Some(Symbol(1)),
            Box::new(Statement::Block(vec![Statement::Return(Some(
                Expr::Binary(
                    Box::new(Expr::Int(1)),
                    BinaryOp::Add,
                    Box::new(Expr::Int(1)),
                ),
            ))])),
        );

        assert_eq!(ast, expected_statement);
    }

    #[test]
    fn test_function_declaration_static_modifier() {
        let mut interner = Interner::new();
        let input = "static func i64 sum() -> Parser {return 1 + 1;}";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_statement = Statement::FunctionDeclaration(
            true,
            Type::I64,
            Symbol(0),
            vec![],
            Some(Symbol(1)),
            Box::new(Statement::Block(vec![Statement::Return(Some(
                Expr::Binary(
                    Box::new(Expr::Int(1)),
                    BinaryOp::Add,
                    Box::new(Expr::Int(1)),
                ),
            ))])),
        );

        assert_eq!(ast, expected_statement);
    }

    #[test]
    fn test_struct_definition() {
        let mut interner = Interner::new();
        let input = "struct Lexer {usize position; string input;}";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_statement = Statement::Struct(
            Symbol(0),
            vec![(Type::Usize, Symbol(1)), (Type::String, Symbol(2))],
        );

        assert_eq!(ast, expected_statement);
    }

    #[test]
    fn test_match_1() {
        let mut interner = Interner::new();
        let input = "match (foo) {|Ok(bar) -> {return bar;} |Err(error) -> {printf(error);}};";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_statement = Statement::Expr(Expr::Match(
            Box::new(Expr::Ident(Symbol(0))),
            vec![
                (
                    Pattern::Ok(Box::new(Pattern::Binding(Symbol(1)))),
                    Statement::Block(vec![Statement::Return(Some(Expr::Ident(Symbol(1))))]),
                ),
                (
                    Pattern::Err(Box::new(Pattern::Binding(Symbol(2)))),
                    Statement::Block(vec![Statement::Expr(Expr::FunctionCall(
                        Box::new(Expr::Ident(Symbol(3))),
                        vec![Expr::Ident(Symbol(2))],
                    ))]),
                ),
            ],
        ));

        assert_eq!(ast, expected_statement);
    }

    #[test]
    fn test_match_2() {
        let mut interner = Interner::new();
        let input = "match (foo) {|Foo -> {return bar;} |Bar(bar) -> {printf(bar);}};";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_statement = Statement::Expr(Expr::Match(
            Box::new(Expr::Ident(Symbol(0))),
            vec![
                (
                    Pattern::Binding(Symbol(1)),
                    Statement::Block(vec![Statement::Return(Some(Expr::Ident(Symbol(2))))]),
                ),
                (
                    Pattern::Variant(Symbol(3), Box::new(Pattern::Binding(Symbol(2)))),
                    Statement::Block(vec![Statement::Expr(Expr::FunctionCall(
                        Box::new(Expr::Ident(Symbol(4))),
                        vec![Expr::Ident(Symbol(2))],
                    ))]),
                ),
            ],
        ));

        assert_eq!(ast, expected_statement);
    }

    #[test]
    fn test_defer() {
        let mut interner = Interner::new();
        let input = "defer free(foo);";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_statement = Statement::Defer(Box::new(Statement::Expr(Expr::FunctionCall(
            Box::new(Expr::Ident(Symbol(0))),
            vec![Expr::Ident(Symbol(1))],
        ))));

        assert_eq!(ast, expected_statement);
    }

    #[test]
    fn test_variant_initialization_without_val() {
        let mut interner = Interner::new();
        let input = "foo = Var::Bar;";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_statement = Statement::Expr(Expr::Binary(
            Box::new(Expr::Ident(Symbol(0))),
            BinaryOp::Assign,
            Box::new(Expr::Variant(
                Box::new(Expr::Ident(Symbol(1))),
                Pattern::Binding(Symbol(2)),
            )),
        ));

        assert_eq!(ast, expected_statement);
    }

    #[test]
    fn test_variant_initialization_with_val() {
        let mut interner = Interner::new();
        let input = "foo = Var::Bar(bar);";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_statement = Statement::Expr(Expr::Binary(
            Box::new(Expr::Ident(Symbol(0))),
            BinaryOp::Assign,
            Box::new(Expr::Variant(
                Box::new(Expr::Ident(Symbol(1))),
                Pattern::Variant(Symbol(2), Box::new(Pattern::Binding(Symbol(3)))),
            )),
        ));

        assert_eq!(ast, expected_statement);
    }

    #[test]
    fn test_variant_initialization_none() {
        let mut interner = Interner::new();
        let input = "foo = None;";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_statement = Statement::Expr(Expr::Binary(
            Box::new(Expr::Ident(Symbol(0))),
            BinaryOp::Assign,
            Box::new(Expr::None),
        ));

        assert_eq!(ast, expected_statement);
    }

    #[test]
    fn test_variant_initialization_ok() {
        let mut interner = Interner::new();
        let input = "foo = Ok(bar);";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_statement = Statement::Expr(Expr::Binary(
            Box::new(Expr::Ident(Symbol(0))),
            BinaryOp::Assign,
            Box::new(Expr::Ok(Box::new(Expr::Ident(Symbol(1))))),
        ));

        assert_eq!(ast, expected_statement);
    }

    #[test]
    fn test_variant_initialization_some() {
        let mut interner = Interner::new();
        let input = "foo = Some(bar);";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast = parser.parse_statement();

        let expected_statement = Statement::Expr(Expr::Binary(
            Box::new(Expr::Ident(Symbol(0))),
            BinaryOp::Assign,
            Box::new(Expr::Some(Box::new(Expr::Ident(Symbol(1))))),
        ));

        assert_eq!(ast, expected_statement);
    }

    #[test]
    fn test_variant_initialization_err() {
        let mut interner = Interner::new();
        let input = "foo = Err(bar);";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast1 = parser.parse_statement();

        let expected_statement = Statement::Expr(Expr::Binary(
            Box::new(Expr::Ident(Symbol(0))),
            BinaryOp::Assign,
            Box::new(Expr::Err(Box::new(Expr::Ident(Symbol(1))))),
        ));

        assert_eq!(ast1, expected_statement);
    }

    #[test]
    fn test_array_initialization() {
        let mut interner = Interner::new();
        let input = "i64[] foo = {1,2,3};";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast1 = parser.parse_statement();

        let expected_statement = Statement::Declaration(
            Type::FixArray(Box::new(Type::I64)),
            Symbol(0),
            Expr::ArrayInit(vec![Expr::Int(1), Expr::Int(2), Expr::Int(3)]),
        );

        assert_eq!(ast1, expected_statement);
    }

    #[test]
    fn test_array_initialization_empty() {
        let mut interner = Interner::new();
        let input = "i64[] foo = {};";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast1 = parser.parse_statement();

        let expected_statement = Statement::Declaration(
            Type::FixArray(Box::new(Type::I64)),
            Symbol(0),
            Expr::ArrayInit(vec![]),
        );

        assert_eq!(ast1, expected_statement);
    }

    #[test]
    fn test_for_loop() {
        let mut interner = Interner::new();
        let input = "for (i32 i = 0; i < 10; i += 1) {printf(i);continue;}";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut parser = Parser::init_parser(&mut lexer);

        let ast1 = parser.parse_statement();

        let expected_statement = Statement::For(
            Box::new(Statement::Declaration(Type::I32, Symbol(0), Expr::Int(0))),
            Expr::Binary(
                Box::new(Expr::Ident(Symbol(0))),
                BinaryOp::LessThen,
                Box::new(Expr::Int(10)),
            ),
            Expr::Binary(
                Box::new(Expr::Ident(Symbol(0))),
                BinaryOp::AddAssign,
                Box::new(Expr::Int(1)),
            ),
            Box::new(Statement::Block(vec![
                Statement::Expr(Expr::FunctionCall(
                    Box::new(Expr::Ident(Symbol(1))),
                    vec![Expr::Ident(Symbol(0))],
                )),
                Statement::Continue,
            ])),
        );

        assert_eq!(ast1, expected_statement);
    }
}
