# Design and Implementation of Programming Languages(CMSC124): Interpreter

## Lexical Structure

### Single-character tokens

| Lexeme | Token Type | Purpose |
|---|---|---|
| `(` | `LEFT_PAREN` | Opens a grouped expression |
| `)` | `RIGHT_PAREN` | Closes a grouped expression |
| `{` | `LEFT_BRACE` | Starts a code block |
| `}` | `RIGHT_BRACE` | Ends a code block |
| `[` | `LEFT_BRACKET` | Opens an array or index expression |
| `]` | `RIGHT_BRACKET` | Closes an array or index expression |
| `,` | `COMMA` | Separates values or arguments |
| `.` | `DOT` | Accesses a property or separates a fractional number |
| `+` | `PLUS` | Addition |
| `-` | `MINUS` | Subtraction |
| `*` | `STAR` | Multiplication |
| `/` | `SLASH` | Division |
| `;` | `SEMICOLON` | Terminates a statement |

### One- or two-character tokens

| Lexeme | Token Type | Purpose |
|---|---|---|
| `!` | `BANG` | Logical negation |
| `!=` | `BANG_EQUAL` | Inequality comparison |
| `=` | `EQUAL` | Assignment |
| `==` | `EQUAL_EQUAL` | Equality comparison |
| `>` | `GREATER` | Greater-than comparison |
| `>=` | `GREATER_EQUAL` | Greater-than-or-equal comparison |
| `<` | `LESS` | Less-than comparison |
| `<=` | `LESS_EQUAL` | Less-than-or-equal comparison |

### Literal tokens

| Lexeme pattern | Token Type | Purpose |
|---|---|---|
| Name such as `total` | `IDENTIFIER` | Names a variable, function, class, or property |
| Text such as `"hello"` | `STRING` | Represents text data |
| Numeric value such as `123` or `3.14` | `NUMBER` | Represents a numeric value |

### Keywords

| Lexeme | Token Type | Purpose |
|---|---|---|
| `and` | `AND` | Logical conjunction |
| `class` | `CLASS` | Declares a class |
| `else` | `ELSE` | Runs an alternate branch |
| `false` | `FALSE` | Boolean false value |
| `for` | `FOR` | Starts a `for` loop |
| `fun` | `FUN` | Declares a function |
| `if` | `IF` | Starts a conditional branch |
| `nil` | `NIL` | Represents the absence of a value |
| `or` | `OR` | Logical disjunction |
| `print` | `PRINT` | Prints a value |
| `return` | `RETURN` | Returns from a function |
| `super` | `SUPER` | Accesses a superclass method |
| `this` | `THIS` | Refers to the current instance |
| `true` | `TRUE` | Boolean true value |
| `var` | `VAR` | Declares a variable |
| `while` | `WHILE` | Starts a `while` loop |

### End-of-input token

| Lexeme | Token Type | Purpose |
|---|---|---|
| End of source | `EOF` | Marks the end of the token stream |

## Week 1 Language Design Decisions
- `{}` creates blocks.
- `[]` is reserved for arrays/indexing.
- `;` ends statements.
- `()` groups expressions and calls functions.
Planned keywords include if, else, for, fun, and var.

## File Structure

| File name | Purpose |
|---|---|
| `src/token.rs` | `for TokenType, Literal, Token` |
| `src/scanner.rs` | `reads the raw source code character by character and turns it into Tokens` |
