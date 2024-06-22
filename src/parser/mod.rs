use crate::error_handler::ErrorHandler;
use crate::ast::ASTNode;

pub fn parse(tokens: &[String]) -> Result<(ASTNode, usize), ErrorHandler> {
    if tokens.is_empty() {
        return Err(ErrorHandler::ParseError("Empty expression".to_string()));
    }

    let mut index: usize = 0;

    if tokens[index] != "(" {
        return Err(ErrorHandler::ParseError("Expected '('. Good luck!".to_string()));
    }

    index += 1;

    // Handle the case of empty parentheses
    if index < tokens.len() && tokens[index] == ")" {
        return Ok((ASTNode::NoOp, index + 1));
    }

    let operator: String = tokens[index].clone();
    index += 1;

    let mut operands: Vec<ASTNode> = Vec::new();

    while index < tokens.len() && tokens[index] != ")" {
        if tokens[index] == "(" {
            let (node, consumed) = parse(&tokens[index..])?;
            operands.push(node);
            index += consumed;
        } else if tokens[index].starts_with('\'') && tokens[index].ends_with('\'') {
            let string_value = tokens[index][1..tokens[index].len() - 1].to_string();
            operands.push(ASTNode::StringValue(string_value));
            index += 1;
        } else {
            operands.push(ASTNode::Value(tokens[index].clone()));
            index += 1;
        }
    }

    if index >= tokens.len() || tokens[index] != ")" {
        return Err(ErrorHandler::ParseError("Expected ')'. Good luck!".to_string()));
    }

    Ok((ASTNode::Operator(operator, operands), index + 1))
}