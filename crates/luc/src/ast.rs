pub enum Statement {
    Print(Expr),
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

pub enum Value {
    String(String),
    Number(f64),
}

impl Value {
    pub fn into_output(self) -> String {
        match self {
            Value::String(value) => value,
            Value::Number(value) => value.to_string(),
        }
    }
}

pub enum Expr {
    Literal(Value),
}
