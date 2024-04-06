use derive_more::Display;
use fx_utils::types::Vec2;

#[derive(Display, Debug, Clone)]
#[display(fmt = "line = {}, col = {}", line, column)]
pub struct LineCol {
    pub line: usize,
    pub column: usize,
}

#[derive(Display, Debug, Clone)]
#[display(fmt = "Range({}, {})", start, end)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct Span {
    pub start: LineCol,
    pub end: LineCol,
}

impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl LineCol {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl Span {
    pub fn new(line_start: Vec2<usize>, line_end: Vec2<usize>) -> Self {
        let start = LineCol::new(line_start.0, line_start.1);
        let end = LineCol::new(line_end.0, line_end.1);

        Self { start, end }
    }
}
