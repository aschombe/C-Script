#[derive(Debug, Clone)]
pub enum Type {
    IntType,
    FloatType,
    BoolType,
    ListType,
    // VoidType,
    // TupleType(Vec<Type>),
    RefType(Box<Type>),
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::IntType => "int".to_string(),
            Type::FloatType => "float".to_string(),
            Type::BoolType => "bool".to_string(),
            Type::ListType => "list".to_string(),
            // Type::VoidType => "void".to_string(),
            // Type::TupleType(t) => {
            //     let mut s: String = "tuple(".to_string();
            //     for t in t {
            //         s += &t.to_string();
            //         s += ", ";
            //     }
            //     s += ")";
            //     s
            // },
            Type::RefType(t) => format!("ref {}", t.to_string()),
        }
    }
}