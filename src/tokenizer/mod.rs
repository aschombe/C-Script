use regex::Regex;

pub fn tokenize(input: &str) -> Vec<String> {
    // Updated regex pattern to include more operators
    let re: Regex = Regex::new(r"(?P<comment>//[^\n]*)|(?P<whitespace>\s+)|(?P<operator>==|!=|<=|>=|[+\-*/%<>])|(?P<assign>=)|(?P<colon>:)|(?P<identifier>[a-zA-Z_][a-zA-Z0-9_]*)|(?P<number>\d+)|(?P<punctuation>[(){};])").unwrap();
   
    let mut tokens: Vec<String> = Vec::new();
    for cap in re.captures_iter(input) {
        if cap.name("comment").is_some() {
            continue; // Ignore comments
        }
        if cap.name("whitespace").is_some() {
            continue; // Ignore whitespace
        }
        if let Some(m) = cap.name("operator") {
            tokens.push(m.as_str().to_string());
        }
        if let Some(m) = cap.name("assign") {
            tokens.push(m.as_str().to_string());
        }
        if let Some(m) = cap.name("colon") {
            tokens.push(m.as_str().to_string());
        }
        if let Some(m) = cap.name("identifier") {
            tokens.push(m.as_str().to_string());
        }
        if let Some(m) = cap.name("number") {
            tokens.push(m.as_str().to_string());
        }
        if let Some(m) = cap.name("punctuation") {
            tokens.push(m.as_str().to_string());
        }
    }
    tokens
}