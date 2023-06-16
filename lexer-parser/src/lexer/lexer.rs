use std::collections::VecDeque;

use crate::preprocessor::Preprocessor;
use crate::utils::structs::{CompileError, CompileMsg, CompileMsgImpl, FileTokPos, TokPos};

use self::token::{Token};


pub struct Lexer<'a> {
    file_name: &'a str,
    bytes: &'a [u8],
    current_index: usize,
    last_tokens: VecDeque<FileTokPos<Token>>,
    errors: Vec<CompileMsg>,
    greater_in_last_token: bool,
    module_directives: Vec<usize>,
    
}

impl<'a> Lexer<'a> {

    // 

    pub fn new(file_name: &'a str, bytes: &'a [u8]) -> Self {
        Self {
            file_name,
            bytes,
            current_index: 0,
            last_tokens: VecDeque::new(),
            errors: Vec::new(),
            greater_in_last_token: false,
            module_directives: Vec::new(),
        }
    }

    
    fn next_token(&mut self) -> Result<Token> {
        // Determines Starting position of lexer

        let starts_line = self.consume_whitespace();
        let token_start = self.current_index;
        
        // If there is a character available to lex, then do it or mark it as the end of the file
        let Some(first_char) = self.next_char();
        if Some(first_char) {
            // Figure out what type of operator it is
            let token_type = match first_char {
                '+' => Token::Plus,
                '*' => Token::Star,
                '/' => Token::Slash,
                '%' => Token::Mod,
                '^' => Token::Caret,
                ';' => Token::SemiColon,
                ':' => Token::Colon,
                ',' => Token::Comma,
                '(' => Token::OpeningParenthesis,
                ')' => Token::ClosingParenthesis,
                '{' => Token::OpeningCurlyBrace,
                '}' => Token::ClosingCurlyBrace,
                '[' => Token::OpeningSquare,
                ']' => Token::ClosingSquare,


                '.' => self.peek_dot(token_start)?,

                '=' | '<' | '>' | '~' => self.peek_equals(token_start, first_char)?,

                '-' => {
                    if self.try_next('-') {
                        return self.comment();
                    } else {
                        TokenType::Minus
                    }
                }

                '\'' | '\"' => self.lex_string(first_char, token_start)?,
                '[' => {
                    if let Some('=') | Some('[') = self.peek_char() {
                        panic!("Long strings are not supported yet.");
                    } else {
                        TokenType::LSquare
                    }
                }

                '0'..='9' => self.lex_full_number(token_start, first_char)?,

                'a'..='z' | 'A'..='Z' | '_' => self.lex_word(first_char),

                _ => return Err(self.error(SyntaxError::InvalidCharacter(first_char))),
            };
            let len = (self.pos - token_start) as u32;
            let token = Token {
                typ: token_type,
                start: token_start,
                len,
            };
            Ok(token)
        } else {
            Ok(self.end_of_file())
        }
}

/// Skips over the characters in a comment.
fn comment(&mut self) -> Result<Token> {
    // TODO multi-line comments
    while let Some(c) = self.next_char() {
        if c == '\n' {
            return self.next_token();
        }
    }
    Ok(self.end_of_file())
}

/// Peeks the next character.
#[must_use]
fn peek_char(&mut self) -> Option<char> {
    self.iter.peek().map(|(_, c)| *c)
}

/// Pops and returns the next character.
fn next_char(&mut self) -> Option<char> {
    match self.iter.next() {
        Some((pos, c)) => {
            self.pos = pos + c.len_utf8();
            if c == '\n' {
                self.linebreaks.push(self.pos);
            }
            Some(c)
        }
        None => None,
    }
}

}
