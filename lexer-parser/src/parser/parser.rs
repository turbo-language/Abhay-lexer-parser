// Import the necessary modules
use crate::lexer::{lexer, token};
use crate::parser::ast::{ASTNode, StatementNode, ExpressionNode};

// Implement the lexer-to-parser integration
pub struct Parser {
    lexer: Lexer<'static>,
    current_token: Option<token>,
}

impl Parser {
    pub fn new(file_name: &'static str, source: &'static str) -> Self {
        let lexer = Lexer::new(file_name, source);
        let current_token = lexer.next_token().ok();
        Self {
            lexer,
            current_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token().ok();
    }

    fn parse_program(&mut self) -> Result<ASTNode, String> {
        let mut nodes = Vec::new();
        while self.current_token.is_some() {
            let statement = self.parse_statement()?;
            nodes.push(statement);
        }
        Ok(ASTNode::Program(nodes))
    }

    fn parse_statement(&mut self) -> Result<ASTNode, String> {
        match self.current_token {
            Some(Token::Assignment) => self.parse_assignment(),
            Some(Token::If) => self.parse_if_statement(),
            // Handle other statement types here
            _ => Err("Invalid statement".to_string()),
        }
    }

    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        match self.current_token {
            Some(Token::Expression) => self.parse_expression_node(),
            // Handle other expression types here
            _ => Err("Invalid expression".to_string()),
        }
    }

   fn parse_assignment(&mut self) -> Result<ASTNode, String> {
    // Parse the assignment statement
    match self.current_token {
        Some(Token::Identifier(identifier)) => {
            self.advance();
            self.expect_token(Token::Assignment)?;
            self.advance();
            let expression = self.parse_expression()?;
            Ok(ASTNode::Statement(Box::new(StatementNode::Assignment(identifier, expression))))
        }
        _ => Err("Expected identifier in assignment statement".to_string()),
    }
}

    fn parse_if_statement(&mut self) -> Result<ASTNode, String> {
        // Parse the if statement
        self.expect_token(Token::If)?;
        self.advance();
        let condition = self.parse_expression()?;
        let body = self.parse_statement()?;
        let else_body = if self.current_token == Some(Token::Else) {
            self.advance();
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };
        Ok(ASTNode::Statement(Box::new(StatementNode::If(
            condition,
            Box::new(body),
            else_body,
        ))))
    }

    fn parse_expression_node(&mut self) -> Result<ASTNode, String> {
        // Parse the expression node
        let binary_op = self.parse_binary_op()?;
        let function_call = self.parse_function_call()?;
        Ok(ASTNode::Expression(Box::new(ExpressionNode {
            binary_op,
            function_call,
        })))
    }

    // Helper function
    fn expect_token(&mut self, expected_token: Token) -> Result<(), String> {
        if self.current_token == Some(expected_token) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", expected_token, self.current_token))
        }
    }


}
