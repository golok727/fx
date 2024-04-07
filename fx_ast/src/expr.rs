use crate::locator::Span;

#[derive(Debug, Clone)]
pub struct CallExpr {
    // calle
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ClosureExpr {
    // fn def
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct MethodCall {
    // method stuff
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct MemberExpr {
    // member stuff
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    // Binary ops
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct AssignmentExpr {
    // assignment stuff
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct UpdateExpr {
    // update and operation stuff += -- ++
    pub span: Span,
}

#[derive(Debug, Clone)]

pub struct RangeExpr {
    // 1..2
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct VariadicExpr {
    // ...
    pub span: Span,
}
