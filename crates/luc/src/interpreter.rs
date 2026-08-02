use crate::ast::{Program, Statement};

pub fn execute(program: Program) -> Vec<String> {
    let mut output = Vec::new();

    for statement in program.into_statements() {
        match statement {
            Statement::Print(value) => output.push(value),
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use crate::ast::{Program, Statement};

    use super::execute;

    #[test]
    fn executes_print_statement() {
        let program = Program::new(vec![Statement::Print(String::from("Olá"))]);

        let output = execute(program);

        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "Olá");
    }

    #[test]
    fn preserves_statement_order_and_utf8() {
        let program = Program::new(vec![
            Statement::Print(String::from("primeira")),
            Statement::Print(String::from("Olá, 世界")),
            Statement::Print(String::from("terceira")),
        ]);

        let output = execute(program);

        assert_eq!(output.len(), 3);
        assert_eq!(output[0], "primeira");
        assert_eq!(output[1], "Olá, 世界");
        assert_eq!(output[2], "terceira");
    }

    #[test]
    fn executes_empty_program_without_output() {
        let program = Program::new(Vec::new());

        let output = execute(program);

        assert_eq!(output.len(), 0);
    }
}
