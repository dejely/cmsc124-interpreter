use create::token::Token; 

/* 
 * Scanner struct and implementation
    source code
        ↓
    scanner.rs
        ↓
    Vec<Token>
 */

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}