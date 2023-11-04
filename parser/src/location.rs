#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
}

impl SourceLocation {
    pub fn advance(&mut self) {
        self.column += 1;
    }

    pub fn newline(&mut self) {
        self.line += 1;
        self.column = 0;
    }
}
