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

Generics próprios, closures, lifetimes nomeados, smart pointers, concorrência, async, macros próprias e unsafe ainda não foram apresentados.
