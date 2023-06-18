
pub mod token;

use token::{Token, Token_lexer};

use crate::Token::*;

use std::iter::Peekable;
use std::str::CharIndices;

pub struct Lexer<'a> {
    file_name: &'a str,
    source: &'a str,
    // bytes: &'a [u8], - later implementations
    current_index: usize,
    errors: Vec<String>,
    iter: Peekable<CharIndices<'a>>,
    newlinebreaks: Vec<usize>,
}

impl<'a> Lexer<'a> {

    pub fn new(file_name: &'a str, source: &'a str,) -> Self {
        Self {
            file_name,
            source,
            // bytes,
            current_index: 0,
            errors: Vec::new(),
            iter: source.char_indices().peekable(),
            newlinebreaks: Vec::new(),
        }
    }

    // Function to look at the next byte in the input stream
    // Should return a U8 byte which is the next one
    fn peek(&self) -> u8 {
        return self.bytes.get(self.current_index + 1);
    }


    // Goes to the next Byte
    fn next_byte(&mut self) -> Option<u8> {
        self.index += 1;
        return self.bytes.get(self.current_index + 1);

    }

    fn peek_type(&mut self) -> Result<Token, String> {
        Ok(self.peek()?.tokenType).ok_or_else(|| Err("Peek failed".to_string()))
    }

    // Returns whether the next token is of the given type.
    fn check_type(&mut self, expected_type: Token) -> Result<Token, String> {
        Ok(self.peek_type()? == expected_type).ok_or_else(|| Err("Mismatched Types".to_string()))
    }
    
    fn next_token(&mut self) -> Result<Token, String> {

        // Determines Starting position of lexer
        let starts_line = self.consume_whitespace();
        let token_start = self.current_index;
        
        // If there is a character available to lex, then do it or mark it as the end of the file
        let Some(first_char) = self.next_char();
        if Some(first_char) {
            // Figure out what type of operator it is
            let token_type = match first_char {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Star,

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
                

                '.' => self.peek_dot()?,

                '=' | '<' | '>' => self.peek_equals(token_start, first_char)?,

                

                '\'' | '\"' => self.lex_string(first_char, token_start)?,
                
                '0'..='9' => self.lex_full_number(token_start, first_char)?,

                'a'..='z' | 'A'..='Z' | '_' => self.lex_word(first_char),

                _ => return Err("Invalid Syntax".to_string()),

                // self.error() in the future once Error Class is implemented
            };
            let len = (self.pos - token_start) as u32;
            let token = Token_lexer {
                token_type,
                start: token_start,
                len,
            };
            Ok(token)
        } else {
            Ok(self.end_of_file())
        }
}

// Returns the next character.
fn next_char(&mut self) -> Option<char> {
    // self.iter.next() line retrieves the next element from the iter field, which is a Peekable<CharIndices<'a>> iterator.
    // It returns an Option<(usize, char)>, representing the next character's position and value if available.
    match self.iter.next() {
        //pos variable holds the byte position of the character, and c holds the character value
        Some((pos, c)) => {
            // New value of the index
            self.current_index = pos + c.len_utf8();
            if c == '\n' {
                // Updates the column count
                self.newlinebreaks.push(pos);
            }
            Some(c)
        }
        None => None,
    }
}

/// Skips over the characters in a comment
/// and Differentiates between / and // 
fn comment(&mut self) -> Result<Token, String> {
    // TODO multi-line comments
    let next_char = self.next_char();
    if next_char == Some('/') {
        Ok(Token::Slash)
    } else {
        while let Some(c) = self.next_char() {
            if c == '/' {
                return self.next_token();
            }
        }
        Ok(Token::Comment)
    }
}



// Peeks the next character in the sequence
#[must_use]
fn peek_char(&mut self) -> Option<char> {
    self.iter.peek().map(|(_, c)| *c)
}

// Function to determine whether the dot is a full decimal number or a single dot
fn peek_dot(&mut self) -> Result<Token, i32> {
    let typ = match self.peek_char() {
    Some(c) if c.is_ascii_digit() => {
        self.next_char();
        self.lex_number_after_decimal()?;
    }
    // So, if the character is not a digit, it will match the _ wildcard pattern
    _ => Token::Dot,
    };
    Ok(typ)
}

 fn peek_equals(&mut self, _tok_start: usize, first_char: char) -> Result<Token, str> {
        if self.try_next('=') {
            let typ = match first_char {
                '=' => CheckEqualSign,
                '=' => NotEqualSign,
                '<' => LessThanEqualSign,
                '>' => GreaterThanEqualSign,
                _ => panic!("peek_equals was called with first_char = {}", first_char),
            };
            Ok(typ)
        } else {
            match first_char {
                '=' => Ok(EqualSign),
                '<' => Ok(LessSign),
                '>' => Ok(GreaterSign),
                _ => panic!("peek_equals was called with first_char = {}", first_char),
            }
        }
    }

}


fn main () {

}