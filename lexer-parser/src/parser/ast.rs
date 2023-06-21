use crate::token::Token;
use crate::lexer::Lexer;

// Define the AST node types
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Statement(Box<StatementNode>),
    Expression(Box<ExpressionNode>),
}

pub struct StatementNode {
    // Define the fields for each statement type
    pub assignment: Option<AssignmentNode>,
    pub if_statement: Option<IfStatementNode>,
    // Add other statement types here
}

pub struct ExpressionNode {
    // Define the fields for each expression type
    pub binary_op: Option<BinaryOpNode>,
    pub function_call: Option<FunctionCallNode>,
    // Add other expression types here
}

pub struct AssignmentNode {
    pub identifier: String,
    pub expression: Box<ExpressionNode>,
}

pub struct IfStatementNode {
    pub condition: Box<ExpressionNode>,
    pub body: Box<StatementNode>,
    pub else_body: Option<Box<StatementNode>>,
}

pub struct BinaryOpNode {
    pub operator: String,
    pub lhs: Box<ExpressionNode>,
    pub rhs: Box<ExpressionNode>,
}

pub struct FunctionCallNode {
    pub name: String,
    pub arguments: Vec<ExpressionNode>,
}

// Implement the conversion from lexer tokens to AST nodes
impl<'a> From<&'a Token> for ASTNode {
    fn from(token: &'a Token) -> Self {
        match token {
            Token::Program => ASTNode::Program(vec![]),
            Token::Statement => ASTNode::Statement(Box::new(StatementNode {
                assignment: None,
                if_statement: None,
            })),
            Token::Expression => ASTNode::Expression(Box::new(ExpressionNode {
                binary_op: None,
                function_call: None,
            })),
            _ => panic!("Invalid conversion from token to AST node"),
        }
    }
}

// Implement the conversion from lexer tokens to AST nodes
pub fn parse_ast(source: &str) -> Result<Vec<ASTNode>, String> {
    let mut lexer = Lexer::new(source);
    let mut ast = Vec::new();

    loop {
        let token = lexer.next_token()?;
        let node: ASTNode = (&token).into();
        ast.push(node);

        if token.is_eof() {
            break;
        }
    }

    Ok(ast)
}
