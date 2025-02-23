use crate::tokenizer::{Token, TokenType};

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    // Program
    Program(Vec<Node>),

    // Declarations
    FunctionDeclaration {
        name: String,
        params: Vec<Parameter>,
        return_type: Option<Box<Node>>,
        body: Box<Node>,
        is_async: bool,
        is_generator: bool,
    },
    VariableDeclaration {
        kind: VariableKind, // let, const, var
        declarations: Vec<VariableDeclarator>,
    },
    ClassDeclaration {
        name: String,
        extends: Option<Box<Node>>,
        implements: Vec<Node>,
        members: Vec<Node>,
    },
    InterfaceDeclaration {
        name: String,
        extends: Vec<Node>,
        members: Vec<Node>,
    },
    TypeAliasDeclaration {
        name: String,
        type_annotation: Box<Node>,
    },

    // Statements
    BlockStatement(Vec<Node>),
    ExpressionStatement(Box<Node>),
    IfStatement {
        condition: Box<Node>,
        then_branch: Box<Node>,
        else_branch: Option<Box<Node>>,
    },
    ForStatement {
        init: Option<Box<Node>>,
        test: Option<Box<Node>>,
        update: Option<Box<Node>>,
        body: Box<Node>,
    },
    WhileStatement {
        test: Box<Node>,
        body: Box<Node>,
    },
    ReturnStatement(Option<Box<Node>>),
    BreakStatement,
    ContinueStatement,
    ThrowStatement(Box<Node>),
    TryStatement {
        block: Box<Node>,
        handler: Option<CatchClause>,
        finalizer: Option<Box<Node>>,
    },

    // Expressions
    Identifier(String),
    Literal {
        token_type: TokenType,
        value: String,
    },
    BinaryExpression {
        left: Box<Node>,
        operator: String,
        right: Box<Node>,
    },
    UnaryExpression {
        operator: String,
        argument: Box<Node>,
    },
    CallExpression {
        callee: Box<Node>,
        arguments: Vec<Node>,
    },
    MemberExpression {
        object: Box<Node>,
        property: Box<Node>,
        computed: bool,
    },
    ArrowFunctionExpression {
        params: Vec<Parameter>,
        body: Box<Node>,
        return_type: Option<Box<Node>>,
    },
    NewExpression {
        callee: Box<Node>,
        arguments: Vec<Node>,
    },
    ThisExpression,
    SuperExpression,

    // Types
    TypeReference {
        name: String,
        type_arguments: Vec<Node>,
    },
    UnionType(Vec<Node>),
    IntersectionType(Vec<Node>),
    FunctionType {
        params: Vec<Parameter>,
        return_type: Box<Node>,
    },
    ObjectType {
        properties: Vec<PropertySignature>,
    },
    ArrayType(Box<Node>),
    TupleType(Vec<Node>),
    LiteralType(Box<Node>),
    LogicalExpression {
        left: Box<Node>,
        operator: String,
        right: Box<Node>,
    },
    PropertySignature {
        key: Box<Node>,
        value: Option<Box<Node>>,
        optional: bool,
    },
    ConditionalExpression {
        test: Box<Node>,
        consequent: Box<Node>,
        alternate: Box<Node>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum VariableKind {
    Let,
    Const,
    Var,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclarator {
    pub id: Box<Node>, // Identifier or Pattern
    pub init: Option<Box<Node>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Option<Box<Node>>,
    pub optional: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PropertySignature {
    pub key: Box<Node>,
    pub value: Option<Box<Node>>,
    pub optional: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CatchClause {
    pub param: Option<Box<Node>>,
    pub body: Box<Node>,
}

impl Iterator for Node {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }
    pub fn parse(&mut self) -> Node {
        self.program()
    }
    fn program(&mut self) -> Node {
        let mut body = Vec::new();
        while !self.is_at_end() {
            body.push(self.declaration());
        }
        Node::Program(body)
    }

    fn declaration(&mut self) -> Node {
        match self.tokens[self.current].token_type {
            TokenType::Function => self.function_declaration(),
            TokenType::Let | TokenType::Const | TokenType::Var => self.variable_declaration(),
            TokenType::Class => {}
            TokenType::Interface => {}
            TokenType::Type => {}
            _ => {}
        }
    }
    fn function_declaration(&mut self) -> Node {
        todo!()
    }
}
