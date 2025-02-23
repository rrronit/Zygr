use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone,Copy)]
pub enum TokenType {
    // Primitive Types
    NumberType,
    StringType,
    BooleanType,
    Null,
    Undefined,
    Any,
    Unknown,
    Never,
    Void,
    BigInt,
    Symbol,
    Object,
    True,
    False,

    // Keywords
    Function,
    Class,
    Interface,
    Let,
    Const,
    Var,
    Type,
    Namespace,
    Module,
    Import,
    Export,
    From,
    As,
    Async,
    Await,
    Return,
    If,
    Else,
    Switch,
    Case,
    Default,
    For,
    While,
    Do,
    Break,
    Continue,
    Throw,
    Try,
    Catch,
    Finally,
    Typeof,
    Instanceof,
    In,
    Of,
    New,
    Extends,
    Implements,
    Get,
    Set,
    Readonly,
    Public,
    Private,
    Protected,
    Static,
    Declare,
    Require,
    Super,
    This,

    // Identifiers & Literals
    Identifier,
    TemplateLiteral,
    NumberLiteral,
    StringLiteral,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Modulo,
    Power,
    Equal,
    EqualEqual,
    TripleEqual,
    Bang,
    BangEqual,
    NotDoubleEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    AmpersandAmpersand,
    PipePipe,
    NullishCoalescing,
    OptionalChain,
    Increment,
    Decrement,
    Spread,
    Arrow,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Semicolon,
    Colon,
    QuestionMark,
    At,
    Backtick,
    LeftAngle,
    RightAngle,

    // Comments
    LineComment,
    BlockComment,

    // Special
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub row: usize,
    pub col: usize,
}

pub struct Tokenizer {
    input: Vec<char>,
    index: usize,
    row: usize,
    col: usize,
}

impl Tokenizer {
    pub fn new(input: String) -> Tokenizer {
        Tokenizer {
            input: input.chars().collect(),
            index: 0,
            row: 1,
            col: 1,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if self.index >= self.input.len() {
            return None;
        }
        let c = self.input[self.index];
        self.index += 1;

        if c == '\n' {
            self.row += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }

        Some(c)
    }

    fn peek_char(&self) -> Option<char> {
        self.input.get(self.index).copied()
    }

    fn peek_next_char(&self) -> Option<char> {
        self.input.get(self.index + 1).copied()
    }

    fn is_eof(&self) -> bool {
        self.index >= self.input.len()
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.is_eof() {
            let c = match self.next_char() {
                Some(ch) => ch,
                None => break,
            };

            if c.is_whitespace() {
                continue;
            }

            let token = match c {
                '/' => self.handle_slash(),
                '"' | '\'' => self.read_string(c),
                '`' => self.read_template_literal(),
                '0'..='9' => self.read_number(c),
                '.' => self.handle_dot(),
                '=' | '!' | '<' | '>' | '&' | '|' | '?' | '+' | '-' | '*' | '%' | '^' | '~' => {
                    self.read_operator(c)
                }
                '(' | ')' | '{' | '}' | '[' | ']' | ',' | ';' | ':' | '@' | '#' | '\\' => {
                    self.read_punctuation(c)
                }
                'a'..='z' | 'A'..='Z' | '_' | '$' => self.read_identifier(c),
                _ => self.create_token(TokenType::EOF, c.to_string()),
            };

            tokens.push(token);
        }

        tokens.push(self.create_token(TokenType::EOF, "EOF".to_string()));
        tokens
    }

    fn handle_slash(&mut self) -> Token {
        match self.peek_char() {
            Some('/') => {
                self.next_char();
                self.read_line_comment()
            }
            Some('*') => {
                self.next_char();
                self.read_block_comment()
            }
            _ => self.read_operator('/'),
        }
    }

    fn handle_dot(&mut self) -> Token {
        if self.peek_char() == Some('.') && self.peek_next_char() == Some('.') {
            self.next_char();
            self.next_char();
            self.create_token(TokenType::Spread, "...".to_string())
        } else {
            self.read_punctuation('.')
        }
    }

    fn read_operator(&mut self, first: char) -> Token {
        let mut value = String::from(first);
        let next = self.peek_char();

        match (first, next) {
            ('=', Some('=')) => {
                self.next_char();
                if self.peek_char() == Some('=') {
                    self.next_char();
                    value.push_str("==");
                    return self.create_token(TokenType::TripleEqual, value);
                }
                value.push('=');
                return self.create_token(TokenType::EqualEqual, value);
            }
            ('!', Some('=')) => {
                self.next_char();
                if self.peek_char() == Some('=') {
                    self.next_char();
                    value.push_str("==");
                    return self.create_token(TokenType::NotDoubleEqual, value);
                }
                value.push('=');
                return self.create_token(TokenType::BangEqual, value);
            }
            ('&', Some('&')) => {
                self.next_char();
                value.push('&');
                return self.create_token(TokenType::AmpersandAmpersand, value);
            }
            ('|', Some('|')) => {
                self.next_char();
                value.push('|');
                return self.create_token(TokenType::PipePipe, value);
            }
            ('?', Some('?')) => {
                self.next_char();
                value.push('?');
                return self.create_token(TokenType::NullishCoalescing, value);
            }
            ('?', Some('.')) => {
                self.next_char();
                value.push('.');
                return self.create_token(TokenType::OptionalChain, value);
            }
            ('+', Some('+')) => {
                self.next_char();
                value.push('+');
                return self.create_token(TokenType::Increment, value);
            }
            ('-', Some('-')) => {
                self.next_char();
                value.push('-');
                return self.create_token(TokenType::Decrement, value);
            }
            ('-', Some('>')) => {
                self.next_char();
                value.push('>');
                return self.create_token(TokenType::Arrow, value);
            }
            ('*', Some('*')) => {
                self.next_char();
                value.push('*');
                return self.create_token(TokenType::Power, value);
            }
            _ => {}
        }

        let token_type = match value.as_str() {
            "=" => TokenType::Equal,
            "!" => TokenType::Bang,
            "<" => TokenType::Less,
            ">" => TokenType::Greater,
            "&" => TokenType::BitwiseAnd,
            "|" => TokenType::BitwiseOr,
            "^" => TokenType::BitwiseXor,
            "~" => TokenType::BitwiseNot,
            "+" => TokenType::Plus,
            "-" => TokenType::Minus,
            "*" => TokenType::Star,
            "/" => TokenType::Slash,
            "%" => TokenType::Modulo,
            _ => TokenType::EOF,
        };

        self.create_token(token_type, value)
    }

    fn read_identifier(&mut self, first: char) -> Token {
        let mut value = String::from(first);
        while let Some(c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' || c == '$' {
                value.push(self.next_char().unwrap());
            } else {
                break;
            }
        }

        let token_type = match value.as_str() {
            // Primitive types
            "number" => TokenType::NumberType,
            "string" => TokenType::StringType,
            "boolean" => TokenType::BooleanType,
            "null" => TokenType::Null,
            "undefined" => TokenType::Undefined,
            "any" => TokenType::Any,
            "unknown" => TokenType::Unknown,
            "never" => TokenType::Never,
            "void" => TokenType::Void,
            "bigint" => TokenType::BigInt,
            "symbol" => TokenType::Symbol,
            "object" => TokenType::Object,
            "true" => TokenType::True,
            "false" => TokenType::False,
            // Keywords
            "function" => TokenType::Function,
            "class" => TokenType::Class,
            "interface" => TokenType::Interface,
            "let" => TokenType::Let,
            "const" => TokenType::Const,
            "var" => TokenType::Var,
            "type" => TokenType::Type,
            "namespace" => TokenType::Namespace,
            "module" => TokenType::Module,
            "import" => TokenType::Import,
            "export" => TokenType::Export,
            "from" => TokenType::From,
            "as" => TokenType::As,
            "async" => TokenType::Async,
            "await" => TokenType::Await,
            "return" => TokenType::Return,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "switch" => TokenType::Switch,
            "case" => TokenType::Case,
            "default" => TokenType::Default,
            "for" => TokenType::For,
            "while" => TokenType::While,
            "do" => TokenType::Do,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "throw" => TokenType::Throw,
            "try" => TokenType::Try,
            "catch" => TokenType::Catch,
            "finally" => TokenType::Finally,
            "typeof" => TokenType::Typeof,
            "instanceof" => TokenType::Instanceof,
            "in" => TokenType::In,
            "of" => TokenType::Of,
            "new" => TokenType::New,
            "extends" => TokenType::Extends,
            "implements" => TokenType::Implements,
            "get" => TokenType::Get,
            "set" => TokenType::Set,
            "readonly" => TokenType::Readonly,
            "public" => TokenType::Public,
            "private" => TokenType::Private,
            "protected" => TokenType::Protected,
            "static" => TokenType::Static,
            "declare" => TokenType::Declare,
            "require" => TokenType::Require,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            _ => TokenType::Identifier,
        };

        self.create_token(token_type, value)
    }

    fn read_number(&mut self, first: char) -> Token {
        let mut value = String::from(first);
        let mut has_dot = first == '.';
        let mut is_exponent = false;

        while let Some(c) = self.peek_char() {
            if c.is_ascii_digit() {
                value.push(self.next_char().unwrap());
            } else if c == '.' && !has_dot {
                has_dot = true;
                value.push(self.next_char().unwrap());
            } else if (c == 'e' || c == 'E') && !is_exponent {
                is_exponent = true;
                value.push(self.next_char().unwrap());
                if let Some(sign) = self.peek_char() {
                    if sign == '+' || sign == '-' {
                        value.push(self.next_char().unwrap());
                    }
                }
            } else if c == '_' {
                self.next_char(); // Skip underscores in numeric literals
            } else {
                break;
            }
        }

        if self.peek_char() == Some('n') {
            value.push(self.next_char().unwrap());
            self.create_token(TokenType::BigInt, value)
        } else {
            self.create_token(TokenType::NumberLiteral, value)
        }
    }

    fn read_string(&mut self, quote: char) -> Token {
        let mut value = String::new();
        while let Some(c) = self.next_char() {
            if c == quote {
                break;
            }
            if c == '\\' {
                if let Some(escaped) = self.next_char() {
                    value.push('\\');
                    value.push(escaped);
                }
            } else {
                value.push(c);
            }
        }
        self.create_token(TokenType::StringLiteral, value)
    }

    fn read_template_literal(&mut self) -> Token {
        let mut value = String::new();
        while let Some(c) = self.next_char() {
            if c == '`' {
                break;
            }
            if c == '$' && self.peek_char() == Some('{') {
                self.next_char();
                value.push_str("${");
            } else if c == '\\' {
                if let Some(escaped) = self.next_char() {
                    value.push('\\');
                    value.push(escaped);
                }
            } else {
                value.push(c);
            }
        }
        self.create_token(TokenType::TemplateLiteral, value)
    }

    fn read_line_comment(&mut self) -> Token {
        let mut value = String::new();
        while let Some(c) = self.next_char() {
            if c == '\n' {
                break;
            }
            value.push(c);
        }
        self.create_token(TokenType::LineComment, value)
    }

    fn read_block_comment(&mut self) -> Token {
        let mut value = String::new();
        let mut prev = '\0';
        while let Some(c) = self.next_char() {
            if prev == '*' && c == '/' {
                break;
            }
            prev = c;
            value.push(c);
        }
        self.create_token(TokenType::BlockComment, value)
    }

    fn read_punctuation(&mut self, c: char) -> Token {
        let token_type = match c {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            ';' => TokenType::Semicolon,
            ':' => TokenType::Colon,
            '?' => TokenType::QuestionMark,
            '@' => TokenType::At,
            '<' => TokenType::LeftAngle,
            '>' => TokenType::RightAngle,
            '`' => TokenType::Backtick,
            _ => TokenType::EOF,
        };
        self.create_token(token_type, c.to_string())
    }

    fn create_token(&self, token_type: TokenType, value: String) -> Token {
        Token {
            token_type,
            value: value.clone(),
            row: self.row,
            col: self.col.saturating_sub(value.len()),
        }
    }
}
