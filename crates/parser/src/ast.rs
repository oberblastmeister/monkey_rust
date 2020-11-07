use lexer::Token;

pub struct Program<'a> {
    statements: Vec<Statement<'a>>,
}

pub enum Statement<'a> {
    Let(LetStmt<'a>),
    Return(ReturnStmt<'a>),
} 

pub struct LetStmt<'a> {
    name: Token<'a>,
    value: Expression<'a>,
}

pub struct ReturnStmt<'a> {
    return_value: Expression<'a>,
}

pub struct ExpressionStmt<'a>(Expression<'a>);

pub enum Expression<'a> {
    Infix(InfixExpression<'a>),
    Prefix(PrefixExpression<'a>),
}

pub struct PrefixExpression<'a> {
    operator: Token<'a>,
    rhs: Box<Expression<'a>>,
}

pub struct InfixExpression<'a> {
    operator: Token<'a>,
    rhs: Box<Expression<'a>>,
}
