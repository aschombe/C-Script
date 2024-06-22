// IfElifElse 
// syntax for IfElifElse:
// if (condition) {
//     // code
// } elif (condition) {
//     // code
// } else {
//     // code
// }
// Switch
// syntax for Switch:
// switch (condition) {
//     case (value) {
//         // code
//     }
//     case (value) {
//         // code
//     }
//     default {
//         // code
//     }
// }
// For
// syntax for For:
// for (<variable>; <condition>; <do something>) {
//     // code
// }
// While
// syntax for While:
// while (condition) {
//     // code
// }
// Function
// syntax for Function:
// func <name>:<return type>(<args>) {
//     // code
// }
// Return
// syntax for Return:
// return <value> (should only be used in functions, breaks out of function)
// Break
// syntax for Break:
// break (should only be used in loops, breaks out of loop)
// Continue
// syntax for Continue:
// continue (should only be used in loops, skips to next iteration)

use crate::types::Types;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Value(Types),    
    NArg(String, Vec<ASTNode>), // add, subtract, multiply, divide, modulo, and, or, concat, print
    OneArg(String, Box<ASTNode>), // neg, sqrt, sin, cos, tan, abs, floor, ceil, not, len, upper, lower, exit
    TwoArg(String, Box<ASTNode>, Box<ASTNode>), // pow, rand
    TwoArgComp(String, Box<ASTNode>, Box<ASTNode>), // eq?, neq?, gt?, lt?, gte?, lte?
    Let(String, Types, Box<ASTNode>), // let <name>:<type> = <value>
    Del(String), // del <name>
    IfElifElse(Box<ASTNode>, Vec<(Box<ASTNode>, Vec<ASTNode>)>, Vec<ASTNode>), // WIP
    Switch(Box<ASTNode>, Vec<(Box<ASTNode>, Vec<ASTNode>)>, Vec<ASTNode>),
    For(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>, Vec<ASTNode>),
    While(Box<ASTNode>, Vec<ASTNode>),
    Function(String, Types, Vec<String>, Vec<ASTNode>),
    Return(Box<ASTNode>),
    Substring(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>), // substring(<string>, <start>, <end>)
    Strip(Box<ASTNode>, Box<ASTNode>), // strip(<string>, <char>)
    Replace(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>), // replace(<string>, <old-char>, <new-char>)
    Break,
    Continue,
}