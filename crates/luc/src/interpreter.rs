use crate::ast::{Expr, Program, Statement, Value};

fn evaluate(expression: Expr) -> Value {
    match expression {
        Expr::Literal(value) => value,
        Expr::Grouping(expression) => evaluate(*expression),
    }
}

pub fn execute(program: Program) -> Vec<String> {
    let mut output = Vec::new();

    for statement in program.into_statements() {
        match statement {
            Statement::Print(expression) => {
                let value = evaluate(expression);
                output.push(value.into_output());
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expr, Program, Statement, Value};

    use super::execute;

    #[test]
    fn preserves_statement_order_and_utf8() {
        let program = Program::new(vec![
            Statement::Print(Expr::Literal(Value::String(String::from("texto")))),
            Statement::Print(Expr::Literal(Value::Number(3.5))),
        ]);

        let output = execute(program);

        assert_eq!(output.len(), 2);
        assert_eq!(output[0], "texto");
        assert_eq!(output[1], "3.5");
    }

    #[test]
    fn executes_empty_program_without_output() {
        let program = Program::new(Vec::new());

        let output = execute(program);

        assert_eq!(output.len(), 0);
    }

    #[test]
    fn evaluates_nested_grouping() {
        let expression = Expr::Grouping(Box::new(Expr::Grouping(Box::new(Expr::Literal(
            Value::String(String::from("agrupado")),
        )))));
        let program = Program::new(vec![Statement::Print(expression)]);

        let output = execute(program);

        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "agrupado");
    }
}
