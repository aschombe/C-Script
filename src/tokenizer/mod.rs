// // the language no longer is lisp-like, it will be more like a C-like language

// // the language will have the following keywords:
// // let, del, if, elif, else, switch, case, default, for, while, function, return, break, continue, substring, strip, replace, print
// // it also has the following operators:
// // +, -, *, /, %, &&, ||, <> (concat), ~ (negate), sqrt, sin, cos, tan, abs, floor, ceil, !, len, upper, lower, exit, ^, rand, ==, !=, >, <, >=, <=
// // the language will have the following data types:
// // int, float, bool, string, void

// // the comments will be the same as in C, // for single line comments and /* */ for multiline comments 

// // examples of the language:
// // let x:int = 5;
// // let y:float = 3.14;
// // let z:string = "hello";
// // let a:bool = true;
// // let x:int = 5 + 3;
// // let y:float = 3.14 * 2;
// // let z:string = "hello" <> "world";
// // del x;
// // if (x > 5) {
// //     print("x is greater than 5");
// // } elif (x < 5) {
// //     print("x is less than 5");
// // } else {
// //     print("x is equal to 5");
// // }
// // switch (x) {
// //     case 1 {
// //         print("x is 1");
// //     }
// //     case 2 {
// //         print("x is 2");
// //     }
// //     default {
// //         print("x is not 1 or 2");
// //     }
// // }
// // let i:int = 0;
// // for (i; i < 10; i++) {
// //     print(i);
// // }
// // let i:int = 0;
// // while (i < 10) {
// //     print(i);
// //     i++;
// // }
// // function
// // func add(x:int, y:int):int {
// //     return x + y;
// // }
// // let x:int = add(5, 3);
// // let y:int = add(3, 2);
// // let z:int = add(2, 1);
// // let x:int = 5;
// // for (x; x < 10; x++) {
// //     print(x);
// //     if (x == 7) {
// //         break;
// //    }
// // }

// // // is a comment
// // /* is a multiline comment */

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
                tokens.pop(); // Remove the previous '/' from tokens
                continue;
            }
            '*' if prev_char == Some('/') => {
                in_multiline_comment = true;
                tokens.pop(); // Remove the previous '/' from tokens
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
            '\n' => {
                if !token.is_empty() {
                    tokens.push(token.clone());
                    token.clear();
                }
                tokens.push(c.to_string());
            }
            ':' | '(' | ')' | '=' => {
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

    tokens
}
