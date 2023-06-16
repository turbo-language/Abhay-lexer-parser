use std::fmt;

// Lifetimes are needed to ensure that the reference of the remains as long the Lexer Struct exists
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Lexer<'a> {
    pub file_name: &'a  String,
    pub tokenType: Token,
    pub start: usize,
    pub len: u32
}


// Allows for easy manipulation and use of the start and end when iterating through the source code
impl Token {
    pub fn range(&self) -> Range<usize> {
        let start = self.start;
        let end = start + self.len as usize;
        start..end
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Def,
    For,
    While,
    Class,
    Return,

    Public,
    Private,

    Print,
    Let,

    Void,
    Int,
    Long,
    Boolean,
    Char,
    Float,

    BooleanValue,
    IntValue,
    LongValue,
    CharValue,
    FloatValue,



    OpeningParenthesis,
    ClosingParenthesis,
    OpeningCurlyBrace,
    ClosingCurlyBrace,
    OpeningSquare,
    ClosingSquare,


    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    EqualSign,
    ExclamationEqualSign,
    LessThanEqualSign,
    GreaterThanEqualSign,
    Comma,
    Colon,
    SemiColon,
    Dot,

    Or,
    And,
    Let,
    If,
    Else,

}



impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Token::Def => write!(f, "Def"),
            Token::For => write!(f, "For"),
            Token::While => write!(f, "While"),
            Token::Class => write!(f, "Class"),
            Token::Return => write!(f, "Return"),
            Token::Public => write!(f, "Public"),
            Token::Private => write!(f, "Private"),
            Token::Print => write!(f, "Print"),
            Token::Let => write!(f, "Let"),
            Token::Void => write!(f, "Void"),
            Token::Int => write!(f, "Int"),
            Token::Long => write!(f, "Long"),
            Token::Boolean => write!(f, "Boolean"),
            Token::Char => write!(f, "Char"),
            Token::Float => write!(f, "Float"),
            Token::BooleanValue => write!(f, "BooleanValue"),
            Token::IntValue => write!(f, "IntValue"),
            Token::LongValue => write!(f, "LongValue"),
            Token::CharValue => write!(f, "CharValue"),
            Token::FloatValue => write!(f, "FloatValue"),
            Token::OpeningParenthesis => write!(f, "("),
            Token::ClosingParenthesis => write!(f, ")"),
            Token::OpeningCurlyBrace => write!(f, "{{"),
            Token::ClosingCurlyBrace => write!(f, "}}"),
            Token::OpeningSquare => write!(f, "["),
            Token::ClosingSquare => write!(f, "]"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Percent => write!(f, "%"),
            Token::EqualSign => write!(f, "="),
            Token::ExclamationEqualSign => write!(f, "!="),
            Token::LessThanEqualSign => write!(f, "<="),
            Token::GreaterThanEqualSign => write!(f, ">="),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::Or => write!(f, "Or"),
            Token::And => write!(f, "And"),
            Token::If => write!(f, "If"),
            Token::Else => write!(f, "Else"),
        }
    }
}
