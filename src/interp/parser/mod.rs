use super::error_handler::ErrorHandler;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Operator(String, Vec<ASTNode>),
    Value(String),
    StringValue(String),
    NoOp,
}

pub fn tokenize(expr: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut token: String = String::new();
    let mut in_string: bool = false;

    for c in expr.chars() {
        match c {
            '(' | ')' if !in_string => {
                if !token.is_empty() {
                    tokens.push(token.clone());
                    token.clear();
                }
                tokens.push(c.to_string());
            }
            '\'' => {
                if in_string {
                    token.push(c);
                    tokens.push(token.clone());
                    token.clear();
                    in_string = false;
                } else {
                    if !token.is_empty() {
                        tokens.push(token.clone());
                        token.clear();
                    }
                    token.push(c);
                    in_string = true;
                }
            }
            ' ' | '\n' | '\t' if !in_string => {  // treat newlines and tabs as spaces
                if !token.is_empty() {
                    tokens.push(token.clone());
                    token.clear();
                }
            }
            _ => {
                token.push(c);
            }
        }
    }

    if !token.is_empty() {
        tokens.push(token);
    }

    tokens
}

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