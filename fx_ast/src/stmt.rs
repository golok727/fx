use crate::ident::Ident;

#[derive(Debug, Clone)]
pub struct FnStmt {
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
pub struct IfStmt {
    /*
     pub test: BinaryExpr,
     pub consequent: BlockStmt | ExprStmt
     pub alternative: BlockStmt | ExprStmt | None
    */
}

#[derive(Debug, Clone)]
pub struct WhileStmt {
    /*
    pub test: BinaryExpr,
    pub body: BlockStmt | ExprStmt
    */
}

#[derive(Debug, Clone)]
pub struct ForStmt {}

#[derive(Debug, Clone)]
pub struct BlockStmt {
    // body: Vec<Stmt>
}

/// Declaration of variables
#[derive(Debug, Clone)]
pub struct LocalStmt {
    /*
     is_const: bool
     declarations: LocalDeclarator[]
    */
}

#[derive(Debug, Clone)]
pub struct ExpressionStmt {
    /*
    expr: Expr
    */
}
