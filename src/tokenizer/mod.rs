pub fn tokenize(expr: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut token: String = String::new();
    let mut in_string: bool = false;
    let mut in_comment: bool = false;
    let mut in_multiline_comment: bool = false;
    let mut prev_char: Option<char> = None;

    for c in expr.chars() {
        if in_comment {
            if c == '\n' {
                in_comment = false;
                tokens.push("\n".to_string());
            }
            continue;
        }

        if in_multiline_comment {
            if prev_char == Some('*') && c == '/' {
                in_multiline_comment = false;
            }
            prev_char = Some(c);
            continue;
        }

        match c {
            '/' if prev_char == Some('/') => {
                in_comment = true;
                tokens.pop();
                continue;
            }
            '*' if prev_char == Some('/') => {
                in_multiline_comment = true;
                tokens.pop();
                continue;
            }
            '/' | '*' => {
                if !token.is_empty() {
                    tokens.push(token.clone());
                    token.clear();
                }
                tokens.push(c.to_string());
            }
            ' ' => {
                if in_string {
                    token.push(c);
                } else {
                    if !token.is_empty() {
                        tokens.push(token.clone());
                        token.clear();
                    }
                }
            }
            '\n' | '\t' => {
                if !token.is_empty() {
                    tokens.push(token.clone());
                    token.clear();
                }
                tokens.push(c.to_string());
            }
            ':' | '(' | ')' | '=' | ';' | ',' => {
                if !token.is_empty() {
                    tokens.push(token.clone());
                    token.clear();
                }
                tokens.push(c.to_string());
            }
            '\'' => {
                if in_string {
                    tokens.push(token.clone());
                    token.clear();
                    in_string = false;
                } else {
                    in_string = true;
                }
            }
            _ => {
                token.push(c);
            }
        }
        prev_char = Some(c);
    }

    if !token.is_empty() {
        tokens.push(token);
    }

    // clean the tokens (strip \n and \t)
    tokens.retain(|x| x != "\n" && x != "\t");

    tokens
}

