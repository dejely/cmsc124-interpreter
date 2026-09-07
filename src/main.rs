mod scanner;
mod token;

use scanner::Scanner;

fn main() {
    println!("Hello, JM & Dejel!");
    let mut scanner = Scanner::new("(){},.-+;/*[]!=><".to_string());
    let tokens = scanner.scan_tokens();
    println!("{tokens:#?}");
}
