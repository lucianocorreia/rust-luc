# Glossário

## Borrowing

Acesso temporário a um valor sem transferência de ownership. O Capítulo 5 introduz o empréstimo compartilhado `&source_path`.

## Crate

Unidade de compilação Rust. `src/main.rs` define atualmente o crate binário `luc`.

## `ExitCode`

Tipo da biblioteca padrão que representa o status devolvido por um processo ao sistema operacional.

## Movimento

Transferência de ownership. Para `String`, move metadados sem copiar o buffer e invalida o uso pelo binding anterior.

## `Option<T>`

Enum que representa presença com `Some(T)` ou ausência com `None`.

## Ownership

Modelo em que um owner possui a responsabilidade pela validade e destruição de um valor.

## Package

Unidade descrita por um `Cargo.toml` com `[package]`. Pode conter um ou mais crates.

## `Result<T, E>`

Enum que representa sucesso com `Ok(T)` ou falha com `Err(E)`.

## `String`

Tipo proprietário, expansível e UTF-8 para texto alocado dinamicamente.

## `str`

Tipo de texto UTF-8 normalmente acessado por referência, como `&str`.

## Workspace

Conjunto de packages coordenados pelo Cargo, com resolução de dependências, lockfile e diretório de build compartilhados.
