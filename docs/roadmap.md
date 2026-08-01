# Roadmap

O curso deverá ter aproximadamente 150 a 200 capítulos, mas não terá uma lista congelada desse tamanho. Detalhar cedo demais transformaria hipóteses arquiteturais em compromissos didáticos e contrariaria a evolução guiada por problemas.

## Horizonte atual

1. Fundação executável e modelo de memória.
2. Leitura de código-fonte e tratamento de falhas.
3. Lexer incremental.
4. Parser e AST mínimos.
5. Interpretador da primeira expressão Luc.

Os guias 1 a 16 cobrem esse horizonte até a execução de `imprima` com strings. O estado implementado pelo estudante continua registrado separadamente em [Progresso](progress.md).

## Horizonte de expressões

1. Expressões literais e representação de valores.
2. Precedência de operadores e agrupamento.
3. Operadores unários e binários.
4. Erros de runtime.
5. `imprima` recebendo expressões.

Os guias 17 a 21 cobrem esse horizonte até expressões aritméticas completas. Ele só estará concluído no software depois que o estudante implementar os capítulos e validar seus exercícios.

## Próximo horizonte

Ainda não será detalhado. Depois da implementação do Capítulo 21, as limitações observadas decidirão entre bindings de variáveis, comparação e booleanos ou melhorias de infraestrutura como spans completos e saída em streaming.

## Ecossistema pretendido

Ao longo do mesmo Cargo Workspace, o Luc deverá desenvolver lexer, parser, AST, interpretador, bytecode, máquina virtual, biblioteca padrão, editor, syntax highlight, LSP, debugger, gerenciador de pacotes, ferramenta de build, REPL e test runner.

Essa lista define destino, não crates. Uma nova unidade de compilação só aparecerá quando responsabilidade, reutilização ou custo de build justificarem sua fronteira.

## Regras de progressão

- Cada capítulo dura entre 30 minutos e 2 horas.
- Poucos conceitos novos entram por capítulo.
- Um conceito surge depois que o código torna sua necessidade perceptível.
- Refatorações preservam a história e explicam alternativas e trade-offs.
- Capítulos futuros são planejados em detalhe somente perto de sua execução.
