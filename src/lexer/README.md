# Lexer

Lexer is a compiler frontend component which translates source code into tokens that the compiler can utilize.

## Token List

### Keywords (Dee-lang keywords + C keywords)

`break`, `const`, `continue`, `default`, `do`, `else`, `extern`, `for`, `goto`, `if`, `return`, `sizeof`, `static`, `struct`, `typedef`, `while`, `func`, `match`, `self`, `defer`, `void`

### Types

`i64`, `i32`, `i16`, `i8`, `u64`, `u32`, `u16`, `u8`, `f64`, `f32`, `usize`, `string`, `Hashmap`, `Arr`, `Variant`, `Maybe`, `Result`, `Some`, `Ok`, `None`, `Err`, `boolean`, `char`

### Operators

`+`, `-`, `*`, `/`, `!`, `|`, `>`, `<`, `.`, `&`, `=`, `^`, `~`, `?`, `%`, `==`, `!=`, `>=`, `<=`, `->`, `&&`, `||`, `>>`, `<<`, `--`, `++`, `-=`, `+=`, `*=`, `/=`, `&=`, `|=`, `^=`, `%=`

### Delimiters/Punctuations

`(`, `)`, `[`, `]`, `{`, `}`, `:`, `;`, 

### Literals

`TokenIntConstant`, `TokenUIntConstant`, `TokenFloatConstant`, `TokenStringConstant`, `TokenIdentifier`

### Special Tokens

EOF, Error, Underscore

## How Does It Work?

- This is a buffer-based lexer which holds all the source code in memory as `&[u8]` in a `Lexer` struct, reads one character at a time into `current_char` field and tracks that with `position` and `read_position` fields by utilizing `read_char()` method.
- `line_offsets` field holds indexes of every line start for error printing which is going to be implemented in future.
- `next_token()` method creates appropriate `Token`s from contiguous characters. For number token creation, lexer supports number base prefix notation e.g. for hexadecimal number `0x10` is a valid format ("16" in decimal).
    - `x`, `X` -> hexadecimal
    - `o`, `O` -> octal
    - `b`, `B` -> binary
- Lexer has unclosed string literal control in `read_string()` method and integer overflow control in `read_number()` method.
- Identifier names are being hold in `interner` field which is a `Interner` struct
- `input` is an array of bytes which means that the source code should be ASCII standard.
- `Token` type hold two fields, `TokenType` and `Span`. `TokenType` specifies the type of that token and `Span` holds start-end position of that token.

## Interner

- Interner is a way to hold string values like identifier names. It maps every value to a number and performs evaluations on this number values for example if two variable has the same name or same string literal value. Because integer comparison is faster than comparing two strings, in this kinda situations interner provides performance or saves memory.

## Error Handling

- `CompileError` holds error message and the span of the token which causes an error. This type is both used in Lexer and Parser for joint error handling.
- Lexer does not panic or abort the program when it faces an `TokenError`, in fact it consumes that token and continues to tokenize for error recovery.
