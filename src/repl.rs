use std::io;
use std::io::BufRead;
use std::io::Write;

use crate::{lexer, token};
pub fn start<R: io::Read, W: io::Write>(reader: R, writer: W) {
    let mut reader = io::BufReader::new(reader);
    let mut writer = io::BufWriter::new(writer);
    loop {
        let mut buf = String::new();
        let result = reader.read_line(&mut buf);
        match result {
            Ok(_) => {
                let mut l = lexer::Lexer::new(buf);
                loop {
                    let next = l.next_token();
                    if next.typ == token::TokenType::Eof {
                        break;
                    }
                    let _ = writer.write(format!("{:?}\n", next).as_bytes());
                    let _ = writer.flush();
                }
            }
            Err(e) => {
                println!("error={}", e);
                break;
            }
        };
    }
}
