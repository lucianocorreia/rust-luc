# Luc

Luc será uma linguagem de script e um ecossistema de ferramentas construídos incrementalmente em Rust. O repositório também é um curso prático e profundo: cada conceito de Rust será introduzido quando um problema real do projeto o exigir.

O software vive em um único Cargo Workspace. Novos crates serão criados somente quando existir uma fronteira de responsabilidade que justifique o custo arquitetural.

## Estado atual

O workspace contém apenas o package binário `luc`, o ponto de entrada da futura ferramenta de linha de comando.

```text
.
├── Cargo.toml
├── crates/
│   └── luc/
│       ├── Cargo.toml
│       └── src/main.rs
└── docs/
    └── capitulos/
```

## Curso

O curso deverá alcançar aproximadamente 150 a 200 capítulos de 30 minutos a 2 horas. Essa é uma faixa, não um roteiro congelado: os capítulos serão decididos conforme as limitações concretas do software aparecerem.

Comece por [Capítulo 1: o primeiro programa](docs/capitulos/01-primeiro-programa.md). Os guias seguintes estão listados em [Progresso](docs/progress.md).

Documentação de apoio:

- [Roadmap](docs/roadmap.md)
- [Arquitetura](docs/architecture.md)
- [Glossário](docs/glossary.md)
- [Conceitos de Rust](docs/rust-concepts.md)

## Comandos

```shell
cargo check --workspace
cargo test --workspace
cargo run -p luc
```

