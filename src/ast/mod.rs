// type Ast = Vec<Expr>;

#[derive(Debug, Clone)]
pub enum Expr {
    // Variables
    Let(String, Box<Expr>, Box<Expr>),
    Set(String, Box<Expr>), // syntax: var_name = expr
    Delete(String),
    VarRef(String),
    
    // Literals
    Type(String),
    String(String),
    // Var(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    List(Vec<Expr>),
    // Tuple(Vec<Expr>),
    Void,
    
    // Arithmetic
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Abs(Box<Expr>),
    
    // Aggregates
    // Min(Vec<Expr>),
    // Max(Vec<Expr>),
    // Sum(Vec<Expr>),
    // Prod(Vec<Expr>),
    // Avg(Vec<Expr>),
    
    // Conditionals
    // if (condition) { body } elif (condition) { body } elif (condition) { body } ... else { body }
    IEE(Box<Expr>, Vec<(Box<Expr>, Box<Expr>)>, Box<Expr>),
    IsEqual(Box<Expr>, Box<Expr>),
    IsLT(Box<Expr>, Box<Expr>),
    IsGT(Box<Expr>, Box<Expr>),
    IsLTE(Box<Expr>, Box<Expr>),
    IsGTE(Box<Expr>, Box<Expr>),
    IsNE(Box<Expr>, Box<Expr>),
    IsZero(Box<Expr>),
    IsNumber(Box<Expr>),
    IsBool(Box<Expr>),
    IsString(Box<Expr>),
    IsList(Box<Expr>),
    IsVoid(Box<Expr>),
    
    // func (recursive - optional) name(arg1: type, arg2: type, ...): type { ... }
    Func(String, bool, Vec<(String, String)>, String, Vec<Expr>),
    FuncApp(String, Vec<Expr>),
    Return(Box<Expr>),

    // For references
    // NewRef(Box<Expr>),
    // Deref(Box<Expr>),
    // SetRef(Box<Expr>, Box<Expr>),

    // For tuples
    // First(Box<Expr>),
    // Second(Box<Expr>),

    // For lists
    // Head(Box<Expr>),
    // Tail(Box<Expr>),
    // Cons(Box<Expr>, Box<Expr>),
    // IsEmpty(Box<Expr>),
    // Len(Box<Expr>),

    WIP(String),
}

impl Expr {
    pub fn to_ast(&self) -> String {
        match self {
            Expr::Let(s, e1, e2) => "Let(".to_string() + s + ", " + &e1.to_ast() + ", " + &e2.to_ast() + ")",
            Expr::Set(s, e) => "Set(".to_string() + s + ", " + &e.to_ast() + ")",
            Expr::Delete(s) => "Delete(".to_string() + s + ")",
            Expr::VarRef(s) => "VarRef(".to_string() + s + ")",

            Expr::Type(t) => "Type".to_string() + t,
            // Expr::Var(s) => "Var".to_string() + s,
            Expr::String(s) => "String".to_string() + s,
            Expr::Int(i) => "Int".to_string() + &i.to_string(),
            Expr::Float(f) => "Float".to_string() + &f.to_string(),
            Expr::Bool(b) => "Bool".to_string() + &b.to_string(),
            Expr::List(l) => {
                let mut s: String = "List(".to_string();
                for e in l {
                    s += &e.to_ast();
                    s += ", ";
                }
                s += ")";
                s
            },
            // Expr::Tuple(t) => {
            //     let mut s: String = "Tuple(".to_string();
            //     for e in t {
            //         s += &e.to_ast();
            //         s += ", ";
            //     }
            //     s += ")";
            //     s
            // },
            Expr::Void => "Void".to_string(),

            Expr::Add(e1, e2) => "Add(".to_string() + &e1.to_ast() + ", " + &e2.to_ast() + ")",
            Expr::Sub(e1, e2) => "Sub(".to_string() + &e1.to_ast() + ", " + &e2.to_ast() + ")",
            Expr::Mul(e1, e2) => "Mul(".to_string() + &e1.to_ast() + ", " + &e2.to_ast() + ")",
            Expr::Div(e1, e2) => "Div(".to_string() + &e1.to_ast() + ", " + &e2.to_ast() + ")",
            Expr::Mod(e1, e2) => "Mod(".to_string() + &e1.to_ast() + ", " + &e2.to_ast() + ")",
            Expr::Pow(e1, e2) => "Pow(".to_string() + &e1.to_ast() + ", " + &e2.to_ast() + ")",
            Expr::Abs(e) => "Abs(".to_string() + &e.to_ast() + ")",

            // Expr::Min(v) => {
            //     let mut s: String = "Min(".to_string();
            //     for e in v {
            //         s += &e.to_ast();
            //         s += ", ";
            //     }
            //     s += ")";
            //     s
            // },
            // Expr::Max(v) => {
            //     let mut s: String = "Max(".to_string();
            //     for e in v {
            //         s += &e.to_ast();
            //         s += ", ";
            //     }
            //     s += ")";
            //     s
            // },
            // Expr::Sum(v) => {
            //     let mut s: String = "Sum(".to_string();
            //     for e in v {
            //         s += &e.to_ast();
            //         s += ", ";
            //     }
            //     s += ")";
            //     s
            // },
            // Expr::Prod(v) => {
            //     let mut s: String = "Prod(".to_string();
            //     for e in v {
            //         s += &e.to_ast();
            //         s += ", ";
            //     }
            //     s += ")";
            //     s
            // },
            // Expr::Avg(v) => {
            //     let mut s: String = "Avg(".to_string();
            //     for e in v {
            //         s += &e.to_ast();
            //         s += ", ";
            //     }
            //     s += ")";
            //     s
            // },

            // Expr::ITE(e1, e2, e3) => "ITE(".to_string() + &e1.to_ast() + ", " + &e2.to_ast() + ", " + &e3.to_ast() + ")",
            Expr::IEE(e1, v, e2) => {
                let mut s: String = "IEE(".to_string() + &e1.to_ast() + ", ";
                for (e1, e2) in v {
                    s += &e1.to_ast();
                    s += ", ";
                    s += &e2.to_ast();
                    s += ", ";
                }
                s += &e2.to_ast();
                s += ")";
                s
            },
            Expr::IsEqual(e1, e2) => "IsEqual(".to_string() + &e1.to_ast() + ", " + &e2.to_ast() + ")",
            Expr::IsLT(e1, e2) => "IsLT(".to_string() + &e1.to_ast() + ", " + &e2.to_ast() + ")",
            Expr::IsGT(e1, e2) => "IsGT(".to_string() + &e1.to_ast() + ", " + &e2.to_ast() + ")",
            Expr::IsLTE(e1, e2) => "IsLTE(".to_string() + &e1.to_ast() + ", " + &e2.to_ast() + ")",
            Expr::IsGTE(e1, e2) => "IsGTE(".to_string() + &e1.to_ast() + ", " + &e2.to_ast() + ")",
            Expr::IsNE(e1, e2) => "IsNE(".to_string() + &e1.to_ast() + ", " + &e2.to_ast() + ")",
            Expr::IsZero(e) => "IsZero(".to_string() + &e.to_ast() + ")",
            Expr::IsNumber(e) => "IsNumber(".to_string() + &e.to_ast() + ")",
            Expr::IsBool(e) => "IsBool(".to_string() + &e.to_ast() + ")",
            Expr::IsString(e) => "IsString(".to_string() + &e.to_ast() + ")",
            Expr::IsList(e) => "IsList(".to_string() + &e.to_ast() + ")",
            Expr::IsVoid(e) => "IsVoid(".to_string() + &e.to_ast() + ")",

            Expr::Func(s, r, v, t, e) => {
                let mut result: String = "Func(".to_string() + s + ", " + &r.to_string() + ", ";
                for (arg, arg_type) in v {
                    result += &arg;
                    result += ": ";
                    result += &arg_type;
                    result += ", ";
                }
                result += t;
                result += ", ";
                for expr in e {
                    result += &expr.to_ast();
                    result += ", ";
                }
                result += ")";
                result
            },
            
            Expr::FuncApp(s, v) => {
                let mut s: String = "FuncApp(".to_string() + s + ", ";
                for e in v {
                    s += &e.to_ast();
                    s += ", ";
                }
                s += ")";
                s
            },
            Expr::Return(e) => "Return(".to_string() + &e.to_ast() + ")",

            // Expr::NewRef(e) => "NewRef(".to_string() + &e.to_ast() + ")",
            // Expr::Deref(e) => "Deref(".to_string() + &e.to_ast() + ")",
            // Expr::SetRef(e1, e2) => "SetRef(".to_string() + &e1.to_ast() + ", " + &e2.to_ast() + ")",

            // Expr::First(e) => "First(".to_string() + &e.to_ast() + ")",
            // Expr::Second(e) => "Second(".to_string() + &e.to_ast() + ")",
        
            // Expr::Head(e) => "Head(".to_string() + &e.to_ast() + ")",
            // Expr::Tail(e) => "Tail(".to_string() + &e.to_ast() + ")",
            // Expr::Cons(e1, e2) => "Cons(".to_string() + &e1.to_ast() + ", " + &e2.to_ast() + ")",
            // Expr::IsEmpty(e) => "IsEmpty(".to_string() + &e.to_ast() + ")",
            // Expr::Len(e) => "Len(".to_string() + &e.to_ast() + ")",

            // _ => "Unknown".to_string(),

            Expr::WIP(s) => "WIP(".to_string() + s + ")",
        }
    }
}