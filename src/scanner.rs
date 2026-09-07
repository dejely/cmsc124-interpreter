use crate::token::Token;

/*
NOTES:

Use `&mut self` in function parameters
for methods that need to change the existing scanner

E.G. function advance changes self.current.
--------------------------------------------
Use `&self` in function parameters
for methods that are only for reading without changing the existing scanner.

E.G. function is_at_end only reads self.current and self.source.len() without changing them.
*/

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

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    // HELPERS BELOW
    /* tell the scanner when it has reached the end of the source code
        e.g. source = ['v', 'a', 'r']
        current = 3
        len = 3

        Thus: current >= source.len() => true
    */
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // reads the next character in the source code and returns it.
    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    // safely looks ahead without moving current.
    fn peek(&self) -> char {
        if self.is_at_end() { '\0' } else { self.source[self.current] }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token>;
}
