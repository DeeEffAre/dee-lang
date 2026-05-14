use std::{collections::HashMap, str::from_utf8};

pub struct Lexer<'a> {
    input: &'a [u8],
    current_char: u8,
    position: usize,
    read_position: usize,
    interner: &'a mut Interner,
    line_offsets: Vec<usize>,
    errors: Vec<CompileError>,
}

impl<'a> Lexer<'a> {
    #[must_use]
    pub fn init_lexer(input: &'a str, interner: &'a mut Interner) -> Self {
        let mut lexer = Lexer {
            input: input.as_bytes(),
            current_char: u8::default(),
            position: 0,
            read_position: 0,
            interner,
            line_offsets: vec![0],
            errors: Vec::new(),
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        self.position = self.read_position;
        if self.read_position >= self.input.len() {
            self.current_char = b'\0'; // EOF
        } else {
            self.current_char = self.input[self.read_position];
        }

        self.read_position += 1;

        if self.current_char == b'\n' {
            self.line_offsets.push(self.read_position) // save index of the line start
        }
    }

    fn skip_whitespaces(&mut self) {
        while self.current_char.is_ascii_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespaces();

        let start = self.position;
        match self.current_char {
            // Two characters
            b'-' if self.input.get(self.read_position) == Some(&b'=') => {
                self.read_char();
                self.read_char();
                Token {
                    kind: TokenType::TokenMinusAssign,
                    span: Span::new(start, self.position),
                }
            }
            b'+' if self.input.get(self.read_position) == Some(&b'=') => {
                self.read_char();
                self.read_char();
                Token {
                    kind: TokenType::TokenPlusAssign,
                    span: Span::new(start, self.position),
                }
            }
            b'*' if self.input.get(self.read_position) == Some(&b'=') => {
                self.read_char();
                self.read_char();
                Token {
                    kind: TokenType::TokenAsteriskAssign,
                    span: Span::new(start, self.position),
                }
            }
            b'/' if self.input.get(self.read_position) == Some(&b'=') => {
                self.read_char();
                self.read_char();
                Token {
                    kind: TokenType::TokenSlashAssign,
                    span: Span::new(start, self.position),
                }
            }
            b'&' if self.input.get(self.read_position) == Some(&b'=') => {
                self.read_char();
                self.read_char();
                Token {
                    kind: TokenType::TokenAmpersandAssign,
                    span: Span::new(start, self.position),
                }
            }
            b'|' if self.input.get(self.read_position) == Some(&b'=') => {
                self.read_char();
                self.read_char();
                Token {
                    kind: TokenType::TokenPipeAssign,
                    span: Span::new(start, self.position),
                }
            }
            b'%' if self.input.get(self.read_position) == Some(&b'=') => {
                self.read_char();
                self.read_char();
                Token {
                    kind: TokenType::TokenModuloAssign,
                    span: Span::new(start, self.position),
                }
            }
            b'^' if self.input.get(self.read_position) == Some(&b'=') => {
                self.read_char();
                self.read_char();
                Token {
                    kind: TokenType::TokenCarretAssign,
                    span: Span::new(start, self.position),
                }
            }
            b'-' if self.input.get(self.read_position) == Some(&b'-') => {
                self.read_char();
                self.read_char();
                Token {
                    kind: TokenType::TokenDecrement,
                    span: Span::new(start, self.position),
                }
            }
            b'+' if self.input.get(self.read_position) == Some(&b'+') => {
                self.read_char();
                self.read_char();
                Token {
                    kind: TokenType::TokenIncrement,
                    span: Span::new(start, self.position),
                }
            }
            b'-' if self.input.get(self.read_position) == Some(&b'>') => {
                self.read_char();
                self.read_char();
                Token {
                    kind: TokenType::TokenArrow,
                    span: Span::new(start, self.position),
                }
            }
            b'!' if self.input.get(self.read_position) == Some(&b'=') => {
                self.read_char();
                self.read_char();
                Token {
                    kind: TokenType::TokenBangEqual,
                    span: Span::new(start, self.position),
                }
            }
            b'=' if self.input.get(self.read_position) == Some(&b'=') => {
                self.read_char();
                self.read_char();
                Token {
                    kind: TokenType::TokenEqual,
                    span: Span::new(start, self.position),
                }
            }
            b'>' if self.input.get(self.read_position) == Some(&b'=') => {
                self.read_char();
                self.read_char();
                Token {
                    kind: TokenType::TokenGreaterEqual,
                    span: Span::new(start, self.position),
                }
            }
            b'<' if self.input.get(self.read_position) == Some(&b'=') => {
                self.read_char();
                self.read_char();
                Token {
                    kind: TokenType::TokenLessEqual,
                    span: Span::new(start, self.position),
                }
            }
            b'&' if self.input.get(self.read_position) == Some(&b'&') => {
                self.read_char();
                self.read_char();
                Token {
                    kind: TokenType::TokenDoubleAmpersand,
                    span: Span::new(start, self.position),
                }
            }
            b'|' if self.input.get(self.read_position) == Some(&b'|') => {
                self.read_char();
                self.read_char();
                Token {
                    kind: TokenType::TokenDoublePipe,
                    span: Span::new(start, self.position),
                }
            }
            b'>' if self.input.get(self.read_position) == Some(&b'>') => {
                self.read_char();
                self.read_char();
                Token {
                    kind: TokenType::TokenBifShiftRight,
                    span: Span::new(start, self.position),
                }
            }
            b'<' if self.input.get(self.read_position) == Some(&b'<') => {
                self.read_char();
                self.read_char();
                Token {
                    kind: TokenType::TokenBifShiftLeft,
                    span: Span::new(start, self.position),
                }
            }

            // One Character
            b'\0' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenEOF,
                    span: Span::new(start, self.position),
                }
            }
            b';' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenSemicolon,
                    span: Span::new(start, self.position),
                }
            }
            b',' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenComma,
                    span: Span::new(start, self.position),
                }
            }
            b'(' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenOpenParen,
                    span: Span::new(start, self.position),
                }
            }
            b')' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenCloseParen,
                    span: Span::new(start, self.position),
                }
            }
            b'[' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenOpenSquare,
                    span: Span::new(start, self.position),
                }
            }
            b']' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenCloseSquare,
                    span: Span::new(start, self.position),
                }
            }
            b'{' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenOpenBrace,
                    span: Span::new(start, self.position),
                }
            }
            b'}' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenCloseBrace,
                    span: Span::new(start, self.position),
                }
            }
            b'+' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenPlus,
                    span: Span::new(start, self.position),
                }
            }
            b'-' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenMinus,
                    span: Span::new(start, self.position),
                }
            }
            b'*' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenAsterisk,
                    span: Span::new(start, self.position),
                }
            }
            b'/' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenSlash,
                    span: Span::new(start, self.position),
                }
            }
            b':' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenColon,
                    span: Span::new(start, self.position),
                }
            }
            b'.' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenDot,
                    span: Span::new(start, self.position),
                }
            }
            b'!' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenBang,
                    span: Span::new(start, self.position),
                }
            }
            b'=' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenAssign,
                    span: Span::new(start, self.position),
                }
            }
            b'>' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenCloseTag,
                    span: Span::new(start, self.position),
                }
            }
            b'<' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenOpenTag,
                    span: Span::new(start, self.position),
                }
            }
            b'&' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenAmpersand,
                    span: Span::new(start, self.position),
                }
            }
            b'|' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenPipe,
                    span: Span::new(start, self.position),
                }
            }
            b'^' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenCarret,
                    span: Span::new(start, self.position),
                }
            }
            b'~' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenTilde,
                    span: Span::new(start, self.position),
                }
            }
            b'?' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenQuestion,
                    span: Span::new(start, self.position),
                }
            }
            b'%' => {
                self.read_char();
                Token {
                    kind: TokenType::TokenModulo,
                    span: Span::new(start, self.position),
                }
            }

            b'"' => self.read_string(),

            b'0'..=b'9' => self.read_number(),

            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.read_identifier_or_keyword(),

            _ => {
                self.errors.push(CompileError {
                    message: "Unknown token".into(),
                    span: Span::new(start, self.position),
                });
                Token {
                    kind: TokenType::TokenError(ErrorId(3)),
                    span: Span::new(start, self.position),
                }
            }
        }
    }

    fn read_identifier_or_keyword(&mut self) -> Token {
        let start = self.position;
        while self.current_char.is_ascii_alphanumeric() || self.current_char == b'_' {
            self.read_char();
        }

        let bytes = &self.input[start..self.position];

        let kind = match bytes {
            // Std C keywords
            b"if" => TokenType::TokenIf,
            b"else" => TokenType::TokenElse,
            b"for" => TokenType::TokenFor,
            b"while" => TokenType::TokenWhile,
            b"do" => TokenType::TokenDo,
            b"return" => TokenType::TokenReturn,
            b"break" => TokenType::TokenBreak,
            b"continue" => TokenType::TokenContinue,
            b"goto" => TokenType::TokenGoto,
            b"struct" => TokenType::TokenStruct,
            b"typedef" => TokenType::TokenTypedef,
            b"const" => TokenType::TokenConst,
            b"static" => TokenType::TokenStatic,
            b"extern" => TokenType::TokenExtern,
            b"sizeof" => TokenType::TokenSizeof,
            b"default" => TokenType::TokenDefault,
            b"void" => TokenType::TokenVoid,
            b"char" => TokenType::TokenChar,

            // Dee-lang specific keywords
            b"match" => TokenType::TokenMatch,
            b"HashMap" => TokenType::TokenHashmap,
            b"Arr" => TokenType::TokenArray,
            b"self" => TokenType::TokenSelf,
            b"Variant" => TokenType::TokenVariant,
            b"Maybe" => TokenType::TokenMaybe,
            b"Result" => TokenType::TokenResult,
            b"bool" => TokenType::TokenBoolean,
            b"func" => TokenType::TokenFunc,

            // Integer types
            b"i64" => TokenType::TokenI64,
            b"i32" => TokenType::TokenI32,
            b"i16" => TokenType::TokenI16,
            b"i8" => TokenType::TokenI8,
            b"u64" => TokenType::TokenU64,
            b"u32" => TokenType::TokenU32,
            b"u16" => TokenType::TokenU16,
            b"u8" => TokenType::TokenU8,
            b"usize" => TokenType::TokenUsize,

            // Float types
            b"f64" => TokenType::TokenF64,
            b"f32" => TokenType::TokenF32,

            b"string" => TokenType::TokenString,

            _ => {
                let symbol = self.interner.intern(bytes);
                TokenType::TokenIdentifier(symbol)
            }
        };

        Token {
            kind,
            span: Span::new(start, self.position),
        }
    }

    fn read_number(&mut self) -> Token {
        const HEX: u8 = 16;
        const DECIMAL: u8 = 10;
        const OCTAL: u8 = 8;
        const BINARY: u8 = 2;

        let start = self.position;

        #[rustfmt::skip]
        let radix = if self.current_char == b'0' {
            match self.input.get(self.read_position) {
                Some(b'x' | b'X') => { self.read_char();self.read_char();HEX }
                Some(b'o' | b'O') => { self.read_char();self.read_char();OCTAL }
                Some(b'b' | b'B') => { self.read_char();self.read_char();BINARY }
                _ => DECIMAL,
            }
        } else { DECIMAL };

        // consume hex and octal digits
        while self.current_char.is_ascii_digit() || {
            let c = self.current_char;
            radix == HEX && c.is_ascii_hexdigit()
        } {
            self.read_char();
        }

        // Is float
        if radix == DECIMAL
            && self.current_char == b'.'
            && self
                .input
                .get(self.read_position)
                .is_some_and(|c| c.is_ascii_digit())
        {
            self.read_char(); // .
            while self.current_char.is_ascii_digit() {
                self.read_char();
            }
            let s = from_utf8(&self.input[start..self.position]).unwrap();
            let v: f64 = s.parse().unwrap_or(0.0);
            return Token {
                kind: TokenType::TokenFloatConstant(v),
                span: Span::new(start, self.position),
            };
        }

        let number_str = from_utf8(&self.input[start..self.position]).expect("123");
        let kind = if radix == DECIMAL {
            if let Ok(v) = number_str.parse::<i64>() {
                TokenType::TokenIntConstant(v)
            } else if let Ok(v) = number_str.parse::<u64>() {
                TokenType::TokenUIntConstant(v)
            } else {
                self.errors.push(CompileError {
                    message: Box::from("Integer overflow"),
                    span: Span::new(start, self.position),
                });
                TokenType::TokenError(ErrorId(1))
            }
        } else {
            let v = u64::from_str_radix(&number_str[2..], radix as u32).expect("invalid literal");
            TokenType::TokenUIntConstant(v)
        };

        Token {
            kind,
            span: Span::new(start, self.position),
        }
    }

    fn read_string(&mut self) -> Token {
        let start = self.position;

        self.read_char(); // opening "

        while self.current_char != b'"' && self.current_char != b'\0' {
            if self.current_char == b'\\' {
                self.read_char();
            }
            self.read_char();
        }

        let kind: TokenType = if self.current_char == b'\0' {
            self.errors.push(CompileError {
                message: Box::from("Unclosed string constant"),
                span: Span::new(start, self.position),
            });
            TokenType::TokenError(ErrorId(2))
        } else {
            self.read_char(); // closing "
            let string_const_bytes = &self.input[start..self.position];
            let symbol = self.interner.intern(string_const_bytes);
            TokenType::TokenStringConstant(symbol)
        };

        Token {
            kind,
            span: Span::new(start, self.position),
        }
    }
}

struct CompileError {
    message: Box<str>,
    span: Span,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    #[must_use]
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Token {
    pub kind: TokenType,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol(pub u32);

// This can move to an arena allocator
// bumpalo crate
pub struct Interner {
    map: HashMap<Box<[u8]>, Symbol>,
    strings: Vec<Box<[u8]>>,
    next_id: u32,
}

impl Interner {
    #[must_use]
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            strings: Vec::new(),
            next_id: 0,
        }
    }

    fn intern(&mut self, name: &[u8]) -> Symbol {
        if let Some(&id) = self.map.get(name) {
            return id;
        }

        let symbol = Symbol(self.strings.len() as u32);
        let boxed_slice: Box<[u8]> = name.into();

        self.map.insert(boxed_slice.clone(), symbol);
        self.strings.push(boxed_slice);
        self.next_id += 1;

        symbol
    }

    pub fn lookup(&self, symbol: Symbol) -> &[u8] {
        &self.strings[symbol.0 as usize]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ErrorId(u32);

#[derive(Debug, Clone, PartialEq, Default)]
pub enum TokenType {
    // Standard C keywords
    TokenBreak,
    TokenChar,
    TokenConst,
    TokenContinue,
    TokenDefault,
    TokenDo,
    TokenElse,
    TokenExtern,
    TokenFor,
    TokenGoto,
    TokenIf,
    TokenReturn,
    TokenSizeof,
    TokenStatic,
    TokenStruct,
    TokenTypedef,
    TokenVoid,
    TokenWhile,

    // function declaration
    TokenFunc,

    // replacement of switch
    TokenMatch,

    // hashmap
    TokenHashmap,

    // array
    TokenArray,

    // method self
    TokenSelf,

    // Variant
    TokenVariant,
    TokenMaybe,
    TokenResult,

    // Error
    TokenError(ErrorId),

    // bool
    TokenBoolean,

    TokenIdentifier(Symbol),
    TokenStringConstant(Symbol),

    // Numbers Constants
    TokenIntConstant(i64),
    TokenUIntConstant(u64),
    TokenFloatConstant(f64),

    // Number Types
    TokenI64,
    TokenI32,
    TokenI16,
    TokenI8,
    TokenU64,
    TokenU32,
    TokenU16,
    TokenU8,
    TokenF64,
    TokenF32,
    TokenUsize,

    // string type
    TokenString,

    // Single Character
    // (
    TokenOpenParen,
    // )
    TokenCloseParen,
    // [
    TokenOpenSquare,
    // ]
    TokenCloseSquare,
    // {
    TokenOpenBrace,
    // }
    TokenCloseBrace,
    // +
    TokenPlus,
    // -
    TokenMinus,
    // *
    TokenAsterisk,
    // /
    TokenSlash,
    // \
    // TokenBackSlash,
    // ;
    TokenSemicolon,
    // :
    TokenColon,
    // .
    TokenDot,
    // !
    TokenBang,
    // |
    TokenPipe,
    // >
    TokenCloseTag,
    // <
    TokenOpenTag,
    // &
    TokenAmpersand,
    // ,
    TokenComma,
    // =
    TokenAssign,
    // ^
    TokenCarret,
    // ~
    TokenTilde,
    // ?
    TokenQuestion,
    // %
    TokenModulo,

    // Two Characters
    // ==
    TokenEqual,
    // !=
    TokenBangEqual,
    // >=
    TokenGreaterEqual,
    // <=
    TokenLessEqual,
    // ->
    TokenArrow,
    // &&
    TokenDoubleAmpersand,
    // ||
    TokenDoublePipe,
    // >>
    TokenBifShiftRight,
    // <<
    TokenBifShiftLeft,
    // --
    TokenDecrement,
    // ++
    TokenIncrement,
    // -=
    TokenMinusAssign,
    // +=
    TokenPlusAssign,
    // *=
    TokenAsteriskAssign,
    // /=
    TokenSlashAssign,
    // |=
    TokenPipeAssign,
    // &=
    TokenAmpersandAssign,
    // ^=
    TokenCarretAssign,
    // %=
    TokenModuloAssign,

    #[default]
    TokenEOF,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tokenize_different_number_bases() {
        let mut interner = Interner::new();
        let input = "123 123.5 0xFF 0o77 0b1001";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut tokens = vec![];
        loop {
            let tok = lexer.next_token();
            if tok.kind == TokenType::TokenEOF {
                tokens.push(tok);
                break;
            } else {
                tokens.push(tok);
            }
        }

        let expected_token_kinds = [
            TokenType::TokenIntConstant(123),
            TokenType::TokenFloatConstant(123.5),
            TokenType::TokenUIntConstant(255),
            TokenType::TokenUIntConstant(63),
            TokenType::TokenUIntConstant(9),
            TokenType::TokenEOF,
        ];

        for (i, token) in tokens.into_iter().enumerate() {
            let expected_token = expected_token_kinds.get(i).unwrap().clone();
            assert_eq!(token.kind, expected_token);
        }
    }

    #[test]
    fn tokenize_a_program() {
        let mut interner = Interner::new();
        let input = "typedef struct {size_t lengthsize_t capacitystring data} Lexer;int main() {	Lexer lex = match (Lexer.init_lexer()) {	| Ok(l) -> l	| Err(e) -> panic(\"Error when initializing lexer\"),	}	Token tok = lex.next_token().unwrap();	return 0;}static Result<string, Error> init_lexer() -> static Lexer {	return Ok(sprintf(\"%s\", \"Hello World!\"))}static Maybe<string> next_token() -> Lexer {	if (self.length + 1 <= self.capacity) {		self.length++;		return Some(self.string);	} else {		None;	}}static Maybe<string> current_token() -> const Lexer {	return Some(self.string);}";
        let mut lexer = Lexer::init_lexer(input, &mut interner);
        let mut tokens = vec![];
        loop {
            let tok = lexer.next_token();
            if tok.kind == TokenType::TokenEOF {
                tokens.push(tok);
                break;
            } else {
                tokens.push(tok);
            }
        }

        let expected_token_kinds = [
            TokenType::TokenTypedef,
            TokenType::TokenStruct,
            TokenType::TokenOpenBrace,
            TokenType::TokenIdentifier(Symbol(0)),
            TokenType::TokenIdentifier(Symbol(1)),
            TokenType::TokenIdentifier(Symbol(2)),
            TokenType::TokenIdentifier(Symbol(3)),
            TokenType::TokenCloseBrace,
            TokenType::TokenIdentifier(Symbol(4)),
            TokenType::TokenSemicolon,
            TokenType::TokenIdentifier(Symbol(5)),
            TokenType::TokenIdentifier(Symbol(6)),
            TokenType::TokenOpenParen,
            TokenType::TokenCloseParen,
            TokenType::TokenOpenBrace,
            TokenType::TokenIdentifier(Symbol(4)),
            TokenType::TokenIdentifier(Symbol(7)),
            TokenType::TokenAssign,
            TokenType::TokenMatch,
            TokenType::TokenOpenParen,
            TokenType::TokenIdentifier(Symbol(4)),
            TokenType::TokenDot,
            TokenType::TokenIdentifier(Symbol(8)),
            TokenType::TokenOpenParen,
            TokenType::TokenCloseParen,
            TokenType::TokenCloseParen,
            TokenType::TokenOpenBrace,
            TokenType::TokenPipe,
            TokenType::TokenIdentifier(Symbol(9)),
            TokenType::TokenOpenParen,
            TokenType::TokenIdentifier(Symbol(10)),
            TokenType::TokenCloseParen,
            TokenType::TokenArrow,
            TokenType::TokenIdentifier(Symbol(10)),
            TokenType::TokenPipe,
            TokenType::TokenIdentifier(Symbol(11)),
            TokenType::TokenOpenParen,
            TokenType::TokenIdentifier(Symbol(12)),
            TokenType::TokenCloseParen,
            TokenType::TokenArrow,
            TokenType::TokenIdentifier(Symbol(13)),
            TokenType::TokenOpenParen,
            TokenType::TokenStringConstant(Symbol(14)),
            TokenType::TokenCloseParen,
            TokenType::TokenComma,
            TokenType::TokenCloseBrace,
            TokenType::TokenIdentifier(Symbol(15)),
            TokenType::TokenIdentifier(Symbol(16)),
            TokenType::TokenAssign,
            TokenType::TokenIdentifier(Symbol(7)),
            TokenType::TokenDot,
            TokenType::TokenIdentifier(Symbol(17)),
            TokenType::TokenOpenParen,
            TokenType::TokenCloseParen,
            TokenType::TokenDot,
            TokenType::TokenIdentifier(Symbol(18)),
            TokenType::TokenOpenParen,
            TokenType::TokenCloseParen,
            TokenType::TokenSemicolon,
            TokenType::TokenReturn,
            TokenType::TokenIntConstant(0),
            TokenType::TokenSemicolon,
            TokenType::TokenCloseBrace,
            TokenType::TokenStatic,
            TokenType::TokenResult,
            TokenType::TokenOpenTag,
            TokenType::TokenString,
            TokenType::TokenComma,
            TokenType::TokenIdentifier(Symbol(19)),
            TokenType::TokenCloseTag,
            TokenType::TokenIdentifier(Symbol(8)),
            TokenType::TokenOpenParen,
            TokenType::TokenCloseParen,
            TokenType::TokenArrow,
            TokenType::TokenStatic,
            TokenType::TokenIdentifier(Symbol(4)),
            TokenType::TokenOpenBrace,
            TokenType::TokenReturn,
            TokenType::TokenIdentifier(Symbol(9)),
            TokenType::TokenOpenParen,
            TokenType::TokenIdentifier(Symbol(20)),
            TokenType::TokenOpenParen,
            TokenType::TokenStringConstant(Symbol(21)),
            TokenType::TokenComma,
            TokenType::TokenStringConstant(Symbol(22)),
            TokenType::TokenCloseParen,
            TokenType::TokenCloseParen,
            TokenType::TokenCloseBrace,
            TokenType::TokenStatic,
            TokenType::TokenMaybe,
            TokenType::TokenOpenTag,
            TokenType::TokenString,
            TokenType::TokenCloseTag,
            TokenType::TokenIdentifier(Symbol(17)),
            TokenType::TokenOpenParen,
            TokenType::TokenCloseParen,
            TokenType::TokenArrow,
            TokenType::TokenIdentifier(Symbol(4)),
            TokenType::TokenOpenBrace,
            TokenType::TokenIf,
            TokenType::TokenOpenParen,
            TokenType::TokenSelf,
            TokenType::TokenDot,
            TokenType::TokenIdentifier(Symbol(23)),
            TokenType::TokenPlus,
            TokenType::TokenIntConstant(1),
            TokenType::TokenLessEqual,
            TokenType::TokenSelf,
            TokenType::TokenDot,
            TokenType::TokenIdentifier(Symbol(24)),
            TokenType::TokenCloseParen,
            TokenType::TokenOpenBrace,
            TokenType::TokenSelf,
            TokenType::TokenDot,
            TokenType::TokenIdentifier(Symbol(23)),
            TokenType::TokenIncrement,
            TokenType::TokenSemicolon,
            TokenType::TokenReturn,
            TokenType::TokenIdentifier(Symbol(25)),
            TokenType::TokenOpenParen,
            TokenType::TokenSelf,
            TokenType::TokenDot,
            TokenType::TokenString,
            TokenType::TokenCloseParen,
            TokenType::TokenSemicolon,
            TokenType::TokenCloseBrace,
            TokenType::TokenElse,
            TokenType::TokenOpenBrace,
            TokenType::TokenIdentifier(Symbol(26)),
            TokenType::TokenSemicolon,
            TokenType::TokenCloseBrace,
            TokenType::TokenCloseBrace,
            TokenType::TokenStatic,
            TokenType::TokenMaybe,
            TokenType::TokenOpenTag,
            TokenType::TokenString,
            TokenType::TokenCloseTag,
            TokenType::TokenIdentifier(Symbol(27)),
            TokenType::TokenOpenParen,
            TokenType::TokenCloseParen,
            TokenType::TokenArrow,
            TokenType::TokenConst,
            TokenType::TokenIdentifier(Symbol(4)),
            TokenType::TokenOpenBrace,
            TokenType::TokenReturn,
            TokenType::TokenIdentifier(Symbol(25)),
            TokenType::TokenOpenParen,
            TokenType::TokenSelf,
            TokenType::TokenDot,
            TokenType::TokenString,
            TokenType::TokenCloseParen,
            TokenType::TokenSemicolon,
            TokenType::TokenCloseBrace,
            TokenType::TokenEOF,
        ];

        for (i, token) in tokens.into_iter().enumerate() {
            let expected_token = expected_token_kinds.get(i).unwrap().clone();
            assert_eq!(token.kind, expected_token);
        }
    }
}
