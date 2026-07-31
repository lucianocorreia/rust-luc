# Roadmap

O curso deverá ter aproximadamente 150 a 200 capítulos, mas não terá uma lista congelada desse tamanho. Detalhar cedo demais transformaria hipóteses arquiteturais em compromissos didáticos e contrariaria a evolução guiada por problemas.

## Horizonte atual

1. Fundação executável e modelo de memória.
2. Leitura de código-fonte e tratamento de falhas.
3. Lexer incremental.
4. Parser e AST mínimos.
5. Interpretador da primeira expressão Luc.

Somente o próximo horizonte será detalhado quando o anterior produzir limitações concretas.

## Ecossistema pretendido

Ao longo do mesmo Cargo Workspace, o Luc deverá desenvolver lexer, parser, AST, interpretador, bytecode, máquina virtual, biblioteca padrão, editor, syntax highlight, LSP, debugger, gerenciador de pacotes, ferramenta de build, REPL e test runner.

Essa lista define destino, não crates. Uma nova unidade de compilação só aparecerá quando responsabilidade, reutilização ou custo de build justificarem sua fronteira.

## Regras de progressão

- Cada capítulo dura entre 30 minutos e 2 horas.
- Poucos conceitos novos entram por capítulo.
- Um conceito surge depois que o código torna sua necessidade perceptível.
- Refatorações preservam a história e explicam alternativas e trade-offs.
- Capítulos futuros são planejados em detalhe somente perto de sua execução.
