use regex::Regex;

pub fn tokenize(input: &str) -> Vec<String> {
    // Updated regex pattern to include string literals
    let re: Regex = Regex::new(r#"(?P<multiline_comment>/\*[\s\S]*?\*/)|(?P<singleline_comment>//[^\n]*)|(?P<whitespace>\s+)|(?P<operator>==|!=|<=|>=|[+\-*/%<>])|(?P<assign>=)|(?P<colon>:)|(?P<identifier>[a-zA-Z_][a-zA-Z0-9_]*)|(?P<number>\d+(\.\d+)?)|(?P<punctuation>[(){};])|(?P<string>"([^"\\]|\\.)*")"#).unwrap();

    let mut tokens: Vec<String> = Vec::new();
    
    for cap in re.captures_iter(input) {
        match (
            cap.name("multiline_comment"),
            cap.name("singleline_comment"),
            cap.name("whitespace"),
            cap.name("operator"),
            cap.name("assign"),
            cap.name("colon"),
            cap.name("identifier"),
            cap.name("number"),
            cap.name("punctuation"),
            cap.name("string")
        ) {
            (Some(_), _, _, _, _, _, _, _, _, _) => continue, // Ignore multiline comments
            (_, Some(_), _, _, _, _, _, _, _, _) => continue, // Ignore single-line comments
            (_, _, Some(_), _, _, _, _, _, _, _) => continue, // Ignore whitespace
            (_, _, _, Some(m), _, _, _, _, _, _) => tokens.push(m.as_str().to_string()), // Operators
            (_, _, _, _, Some(m), _, _, _, _, _) => tokens.push(m.as_str().to_string()), // Assignment
            (_, _, _, _, _, Some(m), _, _, _, _) => tokens.push(m.as_str().to_string()), // Colon
            (_, _, _, _, _, _, Some(m), _, _, _) => tokens.push(m.as_str().to_string()), // Identifiers
            (_, _, _, _, _, _, _, Some(m), _, _) => tokens.push(m.as_str().to_string()), // Numbers
            (_, _, _, _, _, _, _, _, Some(m), _) => tokens.push(m.as_str().to_string()), // Punctuation and Strings
            (_, _, _, _, _, _, _, _, _, Some(m)) => tokens.push(m.as_str().to_string()), // Punctuation
            _ => {} // Ignore unmatched patterns
        }
    }
    
    tokens
}
