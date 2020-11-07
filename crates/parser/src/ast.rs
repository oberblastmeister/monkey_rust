use lexer::Token;

pub struct Program {
    pub statements: Vec<Statement>,
}

pub enum Statement {
    Return(ReturnStmt),
    Let(LetStmt),
    Expr(ExprStmt)
}

pub struct ReturnStmt {

}

pub struct LetStmt {
    Token
}

pub struct ExprStmt(Expr);

pub enum Expr {

}
