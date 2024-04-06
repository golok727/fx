use super::locator::*;

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    /// Functions
    FnStmt(stmt::FnStatement),

    /// local variable declaration
    LocalStmt(stmt::LocalStatement),

    /// if-else
    IfStmt(stmt::IfStatement),

    /// while-loop
    WhileStmt(stmt::WhileStatement),

    /// for-loop
    ForStmt(stmt::ForStatement),

    /// a block statement
    BlockStmt(stmt::BlockStatement),

    /// an expression
    ExprStmt(stmt::ExpressionStatement),
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
    CallExpr,
    //  || {}
    ClosureExpr,

    // a.b.c()
    MethodCall,

    // a.b.c
    MemberExpr,

    // a + b
    BinaryExpr,

    /// a = b
    AssignmentExpr,

    // a++, a--
    UpdateExpr,

    /*
    1..10
    ...args
     */
    RangeExpr,
    VariadicExpr,
}

pub mod expression {}

pub mod stmt {
    use crate::ident::Ident;

    #[derive(Debug, Clone)]

    pub struct FnStatement {
        pub id: Ident,
        /*

        pub variadic: Option<VariadicExpr>
         pub sig: Signature {
           inputs: Params;
           ... stuff
         }

         pub body: BlockStmt | ExprStmt
        */
    }

    #[derive(Debug, Clone)]
    pub struct IfStatement {
        /*
         pub test: BinaryExpr,
         pub consequent: BlockStmt | ExprStmt
         pub alternative: BlockStmt | ExprStmt | None
        */
    }

    #[derive(Debug, Clone)]
    pub struct WhileStatement {
        /*
        pub test: BinaryExpr,
        pub body: BlockStmt | ExprStmt
        */
    }

    #[derive(Debug, Clone)]
    pub struct ForStatement {}

    #[derive(Debug, Clone)]
    pub struct BlockStatement {
        // body: Vec<Stmt>
    }

    /// Declaration of variables
    #[derive(Debug, Clone)]
    pub struct LocalStatement {
        /*
         is_const: bool
         declarations: LocalDeclarator[]
        */
    }

    #[derive(Debug, Clone)]
    pub struct ExpressionStatement {
        /*
        expr: Expr
        */
    }
}
