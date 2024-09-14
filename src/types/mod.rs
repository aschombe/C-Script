#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    String,
    Boolean,
    Void,
    List,
    Tuple,
}

// implement to_string for Type
impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::Float => "float".to_string(),
            Type::String => "string".to_string(),
            Type::Boolean => "bool".to_string(),
            Type::Void => "void".to_string(),
            Type::List => "list".to_string(),
            Type::Tuple => "tuple".to_string(),
        }
    }

    pub fn to_type(value: &str) -> Type {
        match value {
            "int" => Type::Int,
            "float" => Type::Float,
            "string" => Type::String,
            "bool" => Type::Boolean,
            "void" => Type::Void,
            "list" => Type::List,
            "tuple" => Type::Tuple,
            _ => Type::Void,
        }
    }
}