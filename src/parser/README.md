# Parser

Parser is another component of a compiler frontend that parses and creates a tree structure from tokenized source code which obtained from lexer. Dee-lang parser creates an AST instead of a full parse tree.

## How Does It Work?

- `Parser` struct has `lexer` field that holds a reference to a Lexer instance to read tokens.
- Parser creates appropriate AST nodes with evaluating `current_token` and `peeked_token` fields which are instantiated and advanced with `advance()` method.
- Dee-lang parser mixes both Pratt Parser and Recursive Descent Parser with using Pratt Parser in expressions and Recursive Descent Parser in statements.
- `parse_statement()` method is the main parser method, it evaluates the first tokens kind in line and runs appropriate other helper parser methods.
- Main parser elements are `Statement`, `Expr`, `Type`, and `Pattern`.

### Statements

`Block`, `Expr`, `If`, `While`, `DoWhile`, `Return`, `Declaration`, `ShortDeclaration`, `FunctionDeclaration`, `Struct`, `Defer`, `For`, `Break`, `Continue`, `Error`,

### Expressions

`Int`, `Float`, `Binary`, `PrefixUnary`, `PostfixUnary`, `Ternary`, `Ident`, `FunctionCall`, `FieldAccess`, `ArraySub`, `Match`, `Variant`, `None`, `Ok`, `Some`, `Err`, `ArrayInit`, `Error`,

### Types

`I64`, `I32`, `I16`, `I8`, `U64`, `U32`, `U16`, `U8`, `Usize`, `F64`, `F32`, `String`, `Bool`, `Char`, `DynArr`, `FixArray`, `Hashmap`, `Variant`, `Struct`, `Result`, `Maybe`,

### Patterns

`Wildcard`, `None`, `Some`, `Ok`, `Err`, `Variant`, `Binding`,

## Pratt Parser

- Pratt Parser is a parsing strategy that assigns operators binding powers and with this powers it decides which operator have precedence to bind the right-hand expression.

## Error Handling

- `CompileError` holds error message and the span of the token which causes an error. This type is both used in Lexer and Parser for joint error handling.
- Parser does not panic or abort the program when it faces some unknown token patterns, parser methods first synchronize the `current_token` to the closest semicolon or closing brace with `synchronize()` method then returns an Statement::Error or Expr::Error
