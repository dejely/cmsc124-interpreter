# Design and Implementation of Programming Languages(CMSC124): Interpreter

## Lexical Structure

### Single-character tokens

| Lexeme | Token Type | Purpose |
|---|---|---|
| `(` | `LEFT_PAREN` | Opens a grouped expression |
| `)` | `RIGHT_PAREN` | Closes a grouped expression |
| `{` | `LEFT_BRACE` | Starts a code block |
| `}` | `RIGHT_BRACE` | Ends a code block |
| `+` | `PLUS` | Addition |
| `-` | `MINUS` | Subtraction |
| `*` | `STAR` | Multiplication |
| `/` | `SLASH` | Division |
| `=` | `EQUAL` | Assignment |
| `>` | `GREATER_THAN` | Greater-than comparison |
| `<` | `LESS_THAN` | Less-than comparison |
| `;` | `SEMICOLON` | Terminates a statement |
| `,` | `COMMA` | Separates values or arguments |
| `[` | `LEFT_BRACKET` | Opens an array or index expression |
| `]` | `RIGHT_BRACKET` | Closes an array or index expression |
| `!` | `BANG` | Logical negation / prefix for `!=` |


## File Structure

| File name | Purpose |
|---|---|
| `src/tokens.rs` | `for TokenType, Literal, Token` |
| `src/scanner.rs` | `reads the raw source code character by character and turns it into Tokens`| 