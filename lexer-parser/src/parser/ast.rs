use crate::token::Token;

// Define the AST node types
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Statement(Box<StatementNode>),
    Expression(Box<ExpressionNode>),
}

pub struct StatementNode {
    // Define the fields for each statement type
}

pub struct ExpressionNode {
    // Define the fields for each expression type
}

// Implement the AST trait for printing the AST
impl std::fmt::Debug for ASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTNode::Program(nodes) => {
                for node in nodes {
                    write!(f, "{:?}\n", node)?;
                }
                Ok(())
            }
            ASTNode::Statement(node) => write!(f, "{:?}", node),
            ASTNode::Expression(node) => write!(f, "{:?}", node),
        }
    }
}

impl std::fmt::Debug for StatementNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Implement the formatting for each statement type
        // Example:
        // match self {
        //     StatementNode::Assignment(ident, expr) => {
        //         write!(f, "Assignment: {:?} = {:?}", ident, expr)
        //     }
        //     StatementNode::If(condition, body, else_body) => {
        //         write!(f, "If: {:?} {:?} {:?}", condition, body, else_body)
        //     }
        //     ...
        // }
        Ok(())
    }
}

impl std::fmt::Debug for ExpressionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Implement the formatting for each expression type
        // Example:
        // match self {
        //     ExpressionNode::BinaryOp(op, lhs, rhs) => {
        //         write!(f, "BinaryOp: {:?} {:?} {:?}", op, lhs, rhs)
        //     }
        //     ExpressionNode::FunctionCall(name, args) => {
        //         write!(f, "FunctionCall: {:?} {:?}", name, args)
        //     }
        //     ...
        // }
        Ok(())
    }
}

// Implement the conversion from tokens to AST nodes
impl From<Token> for ASTNode {
    fn from(token: Token) -> Self {
        match token {
            Token::Program => ASTNode::Program(vec![]),
            Token::Statement => ASTNode::Statement(Box::new(StatementNode {})),
            Token::Expression => ASTNode::Expression(Box::new(ExpressionNode {})),
            _ => panic!("Invalid conversion from token to AST node"),
        }
    }
}
