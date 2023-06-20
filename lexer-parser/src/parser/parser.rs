// Implement the lexer-to-parser integration
pub struct Parser {
    lexer: Lexer<'static>,
    current_token: Option<Token>,
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
        // Parse the statement based on the current token
        // Example:
        // match self.current_token {
        //     Some(Token::Assignment) => self.parse_assignment(),
        //     Some(Token::If) => self.parse_if_statement(),
        //     ...
        //     _ => Err("Invalid statement".to_string()),
        // }
        unimplemented!()
    }

    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        // Parse the expression based on the current token
        // Example:
        // match self.current_token {
        //     Some(Token::BinaryOp) => self.parse_binary_op(),
        //     Some(Token::FunctionCall) => self.parse_function_call(),
        //     ...
        //     _ => Err("Invalid expression".to_string()),
        // }
        unimplemented!()
    }



    // ---------- Put all of these into seperate parse_functions files 

    // Implement the parsing functions for each statement/expression type

    // fn parse_assignment(&mut self) -> Result<ASTNode, String> {
    //     // Parse the assignment statement
    //     // Example:
    //     // let identifier = self.current_token.expect("Expected identifier");
    //     // self.advance();
    //     // let expression = self.parse_expression()?;
    //     // Ok(ASTNode::Statement(Box::new(StatementNode::Assignment(identifier, expression))))
    //     unimplemented!()
    // }

    // fn parse_if_statement(&mut self) -> Result<ASTNode, String> {
    //     // Parse the if statement
    //     // Example:
    //     // let condition = self.parse_expression()?;
    //     // let body = self.parse_statement()?;
    //     // let else_body = if self.current_token == Some(Token::Else) {
    //     //     self.advance();
    //     //     Some(self.parse_statement()?)
    //     // } else {
    //     //     None
    //     // };
    //     // Ok(ASTNode::Statement(Box::new(StatementNode::If(condition, body, else_body))))
    //     unimplemented!()
    // }

    // Implement the parsing functions for each expression type

    // fn parse_binary_op(&mut self) -> Result<ASTNode, String> {
    //     // Parse the binary operation expression
    //     // Example:
    //     // let operator = self.current_token.expect("Expected binary operator");
    //     // self.advance();
    //     // let lhs = self.parse_expression()?;
    //     // let rhs = self.parse_expression()?;
    //     // Ok(ASTNode::Expression(Box::new(ExpressionNode::BinaryOp(operator, lhs, rhs))))
    //     unimplemented!()
    // }

    // fn parse_function_call(&mut self) -> Result<ASTNode, String> {
    //     // Parse the function call expression
    //     // Example:
    //     // let name = self.current_token.expect("Expected function name");
    //     // self.advance();
    //     // let args = self.parse_arguments()?;
    //     // Ok(ASTNode::Expression(Box::new(ExpressionNode::FunctionCall(name, args))))
    //     unimplemented!()
    // }
}

// Add the Lexer struct definition and implementation here
