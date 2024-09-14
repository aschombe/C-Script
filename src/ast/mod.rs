use crate::types::Type;

#[derive(Debug, Clone)]
pub enum ASTNode {
    // Variables
    // let ident: type = expr;
    Let(String, Type, Box<ASTNode>),
    // set: ident = expr;
    Set(String, Box<ASTNode>),
    // AddEq: ident += expr;
    SetAdd(String, Box<ASTNode>),
    // SubEq: ident -= expr;
    SetSub(String, Box<ASTNode>),
    // MulEq: ident *= expr;
    SetMul(String, Box<ASTNode>),
    // DivEq: ident /= expr;
    SetDiv(String, Box<ASTNode>),
    // ModEq: ident %= expr;
    SetMod(String, Box<ASTNode>),
    // PowEq: ident ^= expr;
    SetPow(String, Box<ASTNode>),
    // del: ident;
    Del(String),
    // Variable Reference
    VarRef(String),

    // Literals
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    // List(Vec<ASTNode>),
    // Tuple(Vec<ASTNode>),

    // Arithmetic
    Add(Box<ASTNode>, Box<ASTNode>),
    Sub(Box<ASTNode>, Box<ASTNode>),
    Mul(Box<ASTNode>, Box<ASTNode>),
    Div(Box<ASTNode>, Box<ASTNode>),
    Mod(Box<ASTNode>, Box<ASTNode>),
    Pow(Box<ASTNode>, Box<ASTNode>),

    // Control Flow
    //  Condition     Elif Conditions and Bodies              Else Body
    IEE(Box<ASTNode>, Vec<(Box<ASTNode>, Vec<Box<ASTNode>>)>, Option<Vec<Box<ASTNode>>>),
    // switch (expr) {
    //   case 1: { body }
    //   case 2: { body }
    //   default: { body }
    // }
    //     Expression    Cases: (Condition, Body)     Default Body
    Switch(Box<ASTNode>, Vec<(ASTNode, Vec<ASTNode>)>, Option<Vec<ASTNode>>),

    // Logical
    And(Box<ASTNode>, Box<ASTNode>),
    Or(Box<ASTNode>, Box<ASTNode>),
    Not(Box<ASTNode>),

    // Comparison
    Eq(Box<ASTNode>, Box<ASTNode>),
    Ne(Box<ASTNode>, Box<ASTNode>),
    Lt(Box<ASTNode>, Box<ASTNode>),
    Gt(Box<ASTNode>, Box<ASTNode>),
    Lte(Box<ASTNode>, Box<ASTNode>),
    Gte(Box<ASTNode>, Box<ASTNode>),

    // Loops
    // for (init (predefined); condition; increment) { body }
    For(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>, Vec<ASTNode>),
    // while (condition) { body }
    While(Box<ASTNode>, Vec<ASTNode>),
    Continue,
    Break,

    // Functions
    FuncDef(String, Vec<(String, Type)>, Type, Vec<ASTNode>),
    FuncCall(String, Vec<ASTNode>),
    Return(Box<ASTNode>),

    // Unary
    Neg(Box<ASTNode>),

    // Unknown
    Unknown,
}

impl ASTNode {
    pub fn to_ast(&self) -> String {
        self.to_string()
    }

    pub fn to_string(&self) -> String {
        match self {
            ASTNode::Let(ident, typ, expr) => {
                "Let(".to_string() + ident + ", " + &typ.to_string() + ", " + &expr.to_ast() + ")"
            }
            ASTNode::Set(ident, expr) => {
                "Set(".to_string() + ident + ", " + &expr.to_ast() + ")"
            }
            ASTNode::Del(ident) => "Del(".to_string() + ident + ")",
            ASTNode::VarRef(ident) => "VarRef(".to_string() + ident + ")",
            ASTNode::Int(i) => "Int(".to_string() + &i.to_string() + ")",
            ASTNode::Float(f) => "Float(".to_string() + &f.to_string() + ")",
            ASTNode::String(s) => "String(".to_string() + s + ")",
            ASTNode::Boolean(b) => "Boolean(".to_string() + &b.to_string() + ")",

            ASTNode::Add(lhs, rhs) => "Add(".to_string() + &lhs.to_ast() + ", " + &rhs.to_ast() + ")",
            ASTNode::Sub(lhs, rhs) => "Sub(".to_string() + &lhs.to_ast() + ", " + &rhs.to_ast() + ")",
            ASTNode::Mul(lhs, rhs) => "Mul(".to_string() + &lhs.to_ast() + ", " + &rhs.to_ast() + ")",
            ASTNode::Div(lhs, rhs) => "Div(".to_string() + &lhs.to_ast() + ", " + &rhs.to_ast() + ")",
            ASTNode::Mod(lhs, rhs) => "Mod(".to_string() + &lhs.to_ast() + ", " + &rhs.to_ast() + ")",
            ASTNode::Pow(lhs, rhs) => "Pow(".to_string() + &lhs.to_ast() + ", " + &rhs.to_ast() + ")",

            // ASTNode::IEE(cond, elifs, el) => {
            //     "IEE(".to_string() + &cond.to_ast() + ", " + &elifs.to_ast() + ", " + &el.to_ast() + ")"
            // }
            // ASTNode::Switch(expr, cases, def) => {
            //     "Switch(".to_string() + &expr.to_ast() + ", " + &cases.to_ast() + ", " + &def.to_ast() + ")"
            // }

            ASTNode::And(lhs, rhs) => "And(".to_string() + &lhs.to_ast() + ", " + &rhs.to_ast() + ")",
            ASTNode::Or(lhs, rhs) => "Or(".to_string() + &lhs.to_ast() + ", " + &rhs.to_ast() + ")",
            ASTNode::Not(expr) => "Not(".to_string() + &expr.to_ast() + ")",

            ASTNode::Eq(lhs, rhs) => "Eq(".to_string() + &lhs.to_ast() + ", " + &rhs.to_ast() + ")",
            ASTNode::Ne(lhs, rhs) => "Ne(".to_string() + &lhs.to_ast() + ", " + &rhs.to_ast() + ")",
            ASTNode::Lt(lhs, rhs) => "Lt(".to_string() + &lhs.to_ast() + ", " + &rhs.to_ast() + ")",
            ASTNode::Gt(lhs, rhs) => "Gt(".to_string() + &lhs.to_ast() + ", " + &rhs.to_ast() + ")",
            ASTNode::Lte(lhs, rhs) => "Lte(".to_string() + &lhs.to_ast() + ", " + &rhs.to_ast() + ")",
            ASTNode::Gte(lhs, rhs) => "Gte(".to_string() + &lhs.to_ast() + ", " + &rhs.to_ast() + ")",

            // ASTNode::For(init, cond, inc, body) => {
            //     "For(".to_string() + &init.to_ast() + ", " + &cond.to_ast() + ", " + &inc.to_ast() + ", " + &body.to_ast() + ")"
            // }
            // ASTNode::While(cond, body) => {
            //     "While(".to_string() + &cond.to_ast() + ", " + &body.to_ast() + ")"
            // }
            ASTNode::Continue => "Continue".to_string(),
            ASTNode::Break => "Break".to_string(),

            // ASTNode::FuncDef(ident, params, ret, body) => {
            //     "FuncDef(".to_string() + ident + ", " + &params.to_ast() + ", " + &ret.to_string() + ", " + &body.to_ast() + ")"
            // }
            // ASTNode::FuncCall(ident, args) => {
            //     "FuncCall(".to_string() + ident + ", " + &args.to_ast() + ")"
            // }
            ASTNode::Return(expr) => "Return(".to_string() + &expr.to_ast() + ")",

            _ => "Unknown".to_string(),
        }
    }
}
