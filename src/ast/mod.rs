use crate::types::Type;

use crate::types::TypeTag;
use crate::keywords::Keywords;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Value(Type),    
    NArg(Keywords, Vec<ASTNode>), // add, subtract, multiply, divide, modulo, and, or, concat, print
    OneArg(String, Box<ASTNode>), // neg, sqrt, sin, cos, tan, abs, floor, ceil, not, len, upper, lower, exit
    TwoArg(String, Box<ASTNode>, Box<ASTNode>), // pow, rand
    TwoArgComp(String, Box<ASTNode>, Box<ASTNode>), // eq?, neq?, gt?, lt?, gte?, lte?
    Let(String, TypeTag, Box<ASTNode>), // let <name>:<type> = <value>
    Del(String), // del <name>
    If(Box<ASTNode>, Vec<ASTNode>, Vec<(Box<ASTNode>, Vec<ASTNode>)>, Vec<ASTNode>),
    Switch(Box<ASTNode>, Vec<(Box<ASTNode>, Vec<ASTNode>)>, Vec<ASTNode>),
    For(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>, Vec<ASTNode>),
    While(Box<ASTNode>, Vec<ASTNode>),
    Function(String, Vec<(String, Type)>, TypeTag, Vec<ASTNode>), // function <name>(<args>):<return-type> { <code> }
    FunctionCall(String, Vec<ASTNode>), // <name>(<args>
    VariableSet(String, Box<ASTNode>), // <name> = <value>
    VariableRef(String), // <name>
    Return(Box<ASTNode>),
    Substring(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>), // substring(<string>, <start>, <end>)
    Strip(Box<ASTNode>, Box<ASTNode>), // strip(<string>, <char>)
    Replace(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>), // replace(<string>, <old-char>, <new-char>)
    Break,
    Continue,
}

pub enum ASTNodeTypes {
    Value,
    NArg,
    OneArg,
    TwoArg,
    TwoArgComp,
    Let,
    Del,
    IfElifElse,
    Switch,
    Case,
    Default,
    For,
    While,
    Function,
    FunctionCall,
    VariableRef,
    Return,
    Substring,
    Strip,
    Replace,
    Break,
    Continue,
    Print,
}