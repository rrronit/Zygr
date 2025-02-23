#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Number,
    String,
    Boolean,
    Null,
    Undefined,
    Any,
    Unknown,
    Never,
    Void,
    BigInt,
    Symbol,
    Object,
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    Union(Vec<Type>),
    Intersection(Vec<Type>),
    Literal(String),
    Array(Box<Type>),
    Tuple(Vec<Type>),
    Custom(String), // For user-defined types
}