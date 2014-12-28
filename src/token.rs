use std::string;

#[deriving(Copy,Clone,PartialEq,Show)]
pub enum AxisName {
    Ancestor,
    AncestorOrSelf,
    Attribute,
    Child,
    Descendant,
    DescendantOrSelf,
    Following,
    FollowingSibling,
    Namespace,
    Parent,
    Preceding,
    PrecedingSibling,
    Self,
}

#[deriving(Clone,PartialEq,Show)]
pub enum NodeTestName {
    Comment,
    Text,
    ProcessingInstruction(string::String),
    Node,
}

#[deriving(PartialEq,Show,Clone)]
pub enum Token {
    And,
    AtSign,
    CurrentNode,
    Divide,
    DoubleSlash,
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    LeftBracket,
    LeftParen,
    LessThan,
    LessThanOrEqual,
    Literal(string::String),
    MinusSign,
    Multiply,
    NotEqual,
    Number(f64),
    Or,
    ParentNode,
    Pipe,
    PlusSign,
    Remainder,
    RightBracket,
    RightParen,
    Slash,

    // Specializations
    Axis(AxisName),
    Function(string::String),
    NameTest((Option<string::String>, string::String)),
    NodeTest(NodeTestName),
    Variable(string::String),
}

impl Token {
    pub fn precedes_node_test(& self) -> bool {
        match *self {
            Token::AtSign |
            Token::Axis(..) => true,
            _ => false,
        }
    }

    pub fn precedes_expression(& self) -> bool {
        match *self {
            Token::LeftParen |
            Token::LeftBracket => true,
            _ => false,
        }
    }

    pub fn is_operator(& self) -> bool {
        match *self {
            Token::Slash |
            Token::DoubleSlash |
            Token::PlusSign |
            Token::MinusSign |
            Token::Pipe |
            Token::Equal |
            Token::NotEqual |
            Token::LessThan |
            Token::LessThanOrEqual |
            Token::GreaterThan |
            Token::GreaterThanOrEqual |
            Token::And |
            Token::Or |
            Token::Remainder |
            Token::Divide |
            Token::Multiply => true,
            _ => false,
        }
    }
}
