pub enum Statement {
    Print(String),
}

pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }

    pub fn into_statements(self) -> Vec<Statement> {
        self.statements
    }
}
