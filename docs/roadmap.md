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

## Horizonte de lógica e estado básico

1. Comparações e igualdade.
2. Operadores lógicos com curto-circuito.
3. Variáveis globais e ambiente de execução.

Os guias 22 a 24 cobrem esse horizonte até permitir reutilização de valores entre instruções e composição de condições.

## Horizonte de controle de fluxo e escopo

1. Condicional `se` e `senao`.
2. Blocos com escopo léxico.
3. Laço `enquanto` com atualização de estado.

Os guias 25 a 27 cobrem esse horizonte até scripts imperativos básicos com decisão e repetição.

## Horizonte de funcoes

1. Declaracao e chamada de funcoes.
2. Parametros e argumentos com validacao de aridade.
3. Retorno de valores com `retorne`.

Os guias 28 a 30 cobrem esse horizonte até a primeira camada de abstração reutilizavel na linguagem.

## Horizonte de stdlib inicial e modulos nativos

1. Valor `nil` e primeiras funcoes nativas.
2. Fronteira interna de biblioteca padrão (`stdlib`).
3. Importacao minima com `use` para modulos nativos.

Os guias 31 a 33 cobrem esse horizonte até uma base de runtime com API nativa inicial e carregamento explicito.

## Próximo horizonte

Ainda não será detalhado. Depois da implementação do Capítulo 33, as limitações observadas decidirão entre closures/funcoes anonimas, estruturas de dados compostas (listas/mapas) ou melhorias de infraestrutura como spans completos e saida em streaming.

## Ecossistema pretendido

Ao longo do mesmo Cargo Workspace, o Luc deverá desenvolver lexer, parser, AST, interpretador, bytecode, máquina virtual, biblioteca padrão, editor, syntax highlight, LSP, debugger, gerenciador de pacotes, ferramenta de build, REPL e test runner.

Essa lista define destino, não crates. Uma nova unidade de compilação só aparecerá quando responsabilidade, reutilização ou custo de build justificarem sua fronteira.

## Regras de progressão

- Cada capítulo dura entre 30 minutos e 2 horas.
- Poucos conceitos novos entram por capítulo.
- Um conceito surge depois que o código torna sua necessidade perceptível.
- Refatorações preservam a história e explicam alternativas e trade-offs.
- Capítulos futuros são planejados em detalhe somente perto de sua execução.
