use super::expr;
use super::locator::*;
use super::stmt;

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    /// Functions
    FnStmt(stmt::FnStmt),

    /// local variable declaration
    LocalStmt(stmt::LocalStmt),

    /// if-else
    IfStmt(stmt::IfStmt),

    /// while-loop
    WhileStmt(stmt::WhileStmt),

    /// for-loop
    ForStmt(stmt::ForStmt),

    /// a block statement
    BlockStmt(stmt::BlockStmt),

    /// an expression
    ExprStmt(stmt::ExpressionStmt),
}

/// Expression
#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    // foo()
    CallExpr(expr::CallExpr),
    //  || {}
    ClosureExpr(expr::ClosureExpr),

    // a.b.c()
    MethodCall(expr::MethodCall),

    // a.b.c
    MemberExpr(expr::MemberExpr),

    // a + b
    BinaryExpr(expr::BinaryExpr),

    /// a = b
    AssignmentExpr(expr::AssignmentExpr),

    // a++, a--
    UpdateExpr(expr::UpdateExpr),

    /*
    1..10
     */
    RangeExpr(expr::RangeExpr),

    /*
    ...args
     */
    VariadicExpr(expr::VariadicExpr),
}
