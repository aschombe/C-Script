// type Ast = Vec<Expr>;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // Variables
    Let(String, Box<Expr>, Box<Expr>),
    Set(String, Box<Expr>), // syntax: var_name = expr
    Delete(String),
    VarRef(String),
    
    Type(String),
    // Literals
    String(String),
    // Var(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    // List(Vec<Expr>),
    // Tuple(Vec<Expr>),
    // Void,
    
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
    // Condition Expressions, list of (condition, body) pairs, and optional else body
    // IEE(Box<Expr>, Option<Vec<(Box<Expr>, Box<Expr>)>>, Option<Vec<Expr>>),
    // If-Condition, If-Body, List of (condition, list of expr) pairs, else body
    IEE(Box<Expr>, Vec<Expr>, Option<Vec<(Expr, Vec<Expr>)>>, Option<Vec<Expr>>),
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
    
    // func name(arg1: type, arg2: type, ...): type { ... }
    Func(String, Vec<(String, String)>, String, Vec<Expr>),
    FuncApp(String, Vec<Expr>),
    Return(Box<Expr>),

    // loops
    // for(var; condition; increment) { body }
    // var, condition, increment, body
    // For(String, Box<Expr>, Box<Expr>, Vec<Expr>),
    For(String, String, String, Vec<Expr>),

    // while(condition) { body }
    While(Box<Expr>, Vec<Expr>),

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
            // Expr::List(l) => {
            //     let mut s: String = "List(".to_string();
            //     for e in l {
            //         s += &e.to_ast();
            //         s += ", ";
            //     }
            //     s += ")";
            //     s
            // },
            // Expr::Tuple(t) => {
            //     let mut s: String = "Tuple(".to_string();
            //     for e in t {
            //         s += &e.to_ast();
            //         s += ", ";
            //     }
            //     s += ")";
            //     s
            // },
            // Expr::Void => "Void".to_string(),

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
            Expr::IEE(e1, v, e2, e3) => {
                let mut result: String = "IEE(".to_string() + &e1.to_ast() + ", ";
                for e in v {
                    result += &e.to_ast();
                    result += ", ";
                }
                if let Some(e2) = e2 {
                    for (e1, e2) in e2 {
                        result += &e1.to_ast();
                        result += ", ";
                        for e in e2 {
                            result += &e.to_ast();
                            result += ", ";
                        }
                    }
                }
                result += ")";
                result
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

            Expr::Func(s, v, t, e) => {
                let mut s: String = "Func(".to_string() + s + ", " + t + ", ";
                for (arg, arg_type) in v {
                    s += arg;
                    s += ": ";
                    s += arg_type;
                    s += ", ";
                }
                for expr in e {
                    s += &expr.to_ast();
                    s += ", ";
                }
                s += ")";
                s
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

            Expr::For(v1, v2, v3, v4) => {
                let mut s: String = "For(".to_string() + v1 + ", " + v2 + ", " + v3 + ", ";
                for e in v4 {
                    s += &e.to_ast();
                    s += ", ";
                }
                s += ")";
                s
            },
            Expr::While(v1, v2) => {
                let mut s: String = "While(".to_string() + &v1.to_ast() + ", ";
                for e in v2 {
                    s += &e.to_ast();
                    s += ", ";
                }
                s += ")";
                s
            }


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

// std::fmt::Display
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Let(s, e1, e2) => write!(f, "Let({}, {}, {})", s, e1, e2),
            Expr::Set(s, e) => write!(f, "Set({}, {})", s, e),
            Expr::Delete(s) => write!(f, "Delete({})", s),
            Expr::VarRef(s) => write!(f, "VarRef({})", s),

            Expr::Type(t) => write!(f, "Type{}", t),
            // Expr::Var(s) => write!(f, "Var{}", s),
            Expr::String(s) => write!(f, "String{}", s),
            Expr::Int(i) => write!(f, "Int{}", i),
            Expr::Float(fl) => write!(f, "Float{}", fl),
            Expr::Bool(b) => write!(f, "Bool{}", b),
            // Expr::List(l) => {
            //     write!(f, "List(")?;
            //     for e in l {
            //         write!(f, "{}, ", e)?;
            //     }
            //     write!(f, ")")
            // },
            // Expr::Tuple(t) => {
            //     write!(f, "Tuple(")?;
            //     for e in t {
            //         write!(f, "{}, ", e)?;
            //     }
            //     write!(f, ")")
            // },
            // Expr::Void => write!(f, "Void"),

            Expr::Add(e1, e2) => write!(f, "Add({}, {})", e1, e2),
            Expr::Sub(e1, e2) => write!(f, "Sub({}, {})", e1, e2),
            Expr::Mul(e1, e2) => write!(f, "Mul({}, {})", e1, e2),
            Expr::Div(e1, e2) => write!(f, "Div({}, {})", e1, e2),
            Expr::Mod(e1, e2) => write!(f, "Mod({}, {})", e1, e2),
            Expr::Pow(e1, e2) => write!(f, "Pow({}, {})", e1, e2),
            Expr::Abs(e) => write!(f, "Abs({})", e),

            // Expr::Min(v) => {
            //     write!(f, "Min(")?;
            //     for e in v {
            //         write!(f, "{}, ", e)?;
            //     }
            //     write!(f, ")")
            // },
            // Expr::Max(v) => {
            //     write!(f, "Max(")?;
            //     for e in v {
            //         write!(f, "{}, ", e)?;
            //     }
            //     write!(f, ")")
            // },
            // Expr::Sum(v) => {
            //     write!(f, "Sum(")?;
            //     for e in v {
            //         write!(f, "{}, ", e)?;
            //     }
            //     write!(f, ")")
            // },
            // Expr::Prod(v) => {
            //     write!(f, "Prod(")?;
            //     for e in v {
            //         write!(f, "{}, ", e)?;
            //     }
            //     write!(f, ")")
            // },
            // Expr::Avg(v) => {
            //     write!(f, "Avg(")?;
            //     for e in v {
            //         write!(f, "{}, ", e)?;
            //     }
            //     write!(f, ")")
            // },

            // Expr::ITE(e1, e2, e3) => write!(f, "ITE({}, {}, {})", e1, e2, e3),
            Expr::IEE(e1, v, e2, e3) => {
                write!(f, "IEE({}, ", e1)?;
                for e in v {
                    write!(f, "{}, ", e)?;
                }
                if let Some(e2) = e2 {
                    for (e1, e2) in e2 {
                        write!(f, "{}, ", e1)?;
                        for e in e2 {
                            write!(f, "{}, ", e)?;
                        }
                    }
                }
                write!(f, ")")
            },
            Expr::IsEqual(e1, e2) => write!(f, "IsEqual({}, {})", e1, e2),
            Expr::IsLT(e1, e2) => write!(f, "IsLT({}, {})", e1, e2),
            Expr::IsGT(e1, e2) => write!(f, "IsGT({}, {})", e1, e2),
            Expr::IsLTE(e1, e2) => write!(f, "IsLTE({}, {})", e1, e2),
            Expr::IsGTE(e1, e2) => write!(f, "IsGTE({}, {})", e1, e2),
            Expr::IsNE(e1, e2) => write!(f, "IsNE({}, {})", e1, e2),
            Expr::IsZero(e) => write!(f, "IsZero({})", e),
            Expr::IsNumber(e) => write!(f, "IsNumber({})", e),
            Expr::IsBool(e) => write!(f, "IsBool({})", e),
            Expr::IsString(e) => write!(f, "IsString({})", e),
            Expr::IsList(e) => write!(f, "IsList({})", e),
            Expr::IsVoid(e) => write!(f, "IsVoid({})", e),
            
            Expr::Func(s, v, t, e) => {
                write!(f, "Func({}, {}, ", s, t)?;
                for (arg, arg_type) in v {
                    write!(f, "{}: {}, ", arg, arg_type)?;
                }
                for expr in e {
                    write!(f, "{}, ", expr)?;
                }
                write!(f, ")")
            },
            Expr::FuncApp(s, v) => {
                write!(f, "FuncApp({}, ", s)?;
                for e in v {
                    write!(f, "{}, ", e)?;
                }
                write!(f, ")")
            },
            Expr::Return(e) => write!(f, "Return({})", e),

            Expr::For(v1, v2, v3, v4) => {
                write!(f, "For({}, {}, {}, ", v1, v2, v3)?;
                for e in v4 {
                    write!(f, "{}, ", e)?;
                }
                write!(f, ")")
            },
            Expr::While(v1, v2) => {
                write!(f, "While({}, ", v1)?;
                for e in v2 {
                    write!(f, "{}, ", e)?;
                }
                write!(f, ")")
            },

            // Expr::NewRef(e) => write!(f, "NewRef({})", e),
            // Expr::Deref(e) => write!(f, "Deref({})", e),
            // Expr::SetRef(e1, e2) => write!(f, "SetRef({}, {})", e1, e2),

            // Expr::First(e) => write!(f, "First({})", e),
            // Expr::Second(e) => write!(f, "Second({})", e),

            // Expr::Head(e) => write!(f, "Head({})", e),
            // Expr::Tail(e) => write!(f, "Tail({})", e),
            // Expr::Cons(e1, e2) => write!(f, "Cons({}, {})", e1, e2),
            // Expr::IsEmpty(e) => write!(f, "IsEmpty({})", e),
            // Expr::Len(e) => write!(f, "Len({})", e),

            // _ => write!(f, "Unknown"),
            Expr::WIP(s) => write!(f, "WIP({})", s),
        }
    }
}

            