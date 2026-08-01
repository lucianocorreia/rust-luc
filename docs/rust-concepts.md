# Conceitos de Rust

Este índice registra onde cada conceito aparece pela primeira vez. “Guia” não significa “implementado”; o estado oficial está em [Progresso](progress.md).

| Conceito | Primeiro capítulo | Motivação no projeto |
| --- | --- | --- |
| Workspace, package e crate | 1 | Criar a unidade executável sem antecipar componentes. |
| `main` e macros de saída fornecidas | 1 | Tornar o binário observável. |
| `String` e ownership | 2 | Receber dados criados em runtime. |
| Movimento | 2 | Entender transferência sem cópia do buffer. |
| Mutabilidade de binding | 3 | Avançar manualmente pela sequência de argumentos. |
| `Option` e `match` | 3 | Representar argumento presente ou ausente. |
| Blocos como expressões | 4 | Produzir um código de saída em cada caminho. |
| `ExitCode` | 4 | Comunicar sucesso ou falha ao shell. |
| `Result` | 5 | Preservar a causa de falhas de leitura. |
| Borrowing compartilhado | 5 | Ler pelo caminho e reutilizá-lo no diagnóstico. |
| `&str` | 6 | Definir uma entrada textual emprestada e flexível para `run`. |
| `()` | 6 | Representar sucesso sem carga útil. |
| Operador `?` | 6 | Propagar falhas sem esconder o caminho principal. |
| `Vec<T>` | 7 | Armazenar tokens na ordem em que aparecem. |
| Iteração sobre `str` | 7 | Percorrer código UTF-8 sem indexar bytes. |
| `Peekable` | 8 | Distinguir operadores simples e compostos. |
| Ownership em structs | 9 | Fazer cada token possuir seu lexema. |
| Cópia de iterador | 10 | Olhar além do ponto sem mover o cursor principal. |
| Trait `Display` | 11 | Apresentar erros estruturados na CLI. |
| `map_err` | 11 | Compor erros de I/O e lexer em `RunError`. |
| Referências mutáveis e dereference | 12 | Atualizar linha e coluna em todos os caminhos do lexer. |
| `usize` | 12 | Representar posições derivadas da entrada em memória. |
| `#[test]`, `#[cfg(test)]` e asserções | 13 | Proteger regras e posições do lexer automaticamente. |
| Indexação de `Vec` | 13 | Inspecionar tokens depois de validar o tamanho nos testes. |
| AST | 14 | Representar o significado sintático sem executar o programa. |
| Métodos consumidores e `into_iter` | 14 | Mover lexemas dos tokens para a AST sem cloná-los. |
| Nó raiz `Program` | 15 | Agrupar instruções e permitir evolução da AST. |
| `vec!` | 16 | Construir programas diretamente nos testes do interpretador. |
| Enums compostos e `f64` | 17 | Representar expressões que produzem strings ou números. |
| Método genérico `parse::<T>` | 17 | Converter lexema numérico para o tipo de runtime. |
| Token sentinela `EOF` | 18 | Posicionar e uniformizar erros no fim do fonte. |
| Tipo `Parser` com `&mut self` | 18 | Encapsular o avanço do cursor entre regras sintáticas. |
| Tipos recursivos e `Box<T>` | 19 | Representar agrupamento com tamanho conhecido. |
| Recursão no parser | 19 | Analisar grupos aninhados usando a própria regra de expressão. |
| Erros de runtime | 20 | Propagar operações inválidas depois do parsing. |
| Padrões de struct e `..` | 20 | Inspecionar campos relevantes de expressões unárias. |
| Precedência e associação | 21 | Construir a árvore correta para operadores binários. |
| Pattern matching em tuplas | 21 | Validar os dois operandos de uma operação. |

Generics próprios, closures, lifetimes nomeados, compartilhamento com `Rc` ou `Arc`, concorrência, async, macros próprias e unsafe ainda não foram apresentados.
