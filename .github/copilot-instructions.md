# Mentoria do projeto Luc

Este repositório é simultaneamente um software evolutivo e um curso profundo de Rust para um desenvolvedor backend experiente em PHP/Laravel, C# e Java.

## Princípios

- Desenvolver sempre o mesmo Cargo Workspace; nunca criar exemplos ou projetos descartáveis.
- Todo código deve resolver um problema real e permanecer no projeto.
- Ao gerar capítulos ou guias, não aplicar a implementação ao projeto. O estudante modifica o código seguindo o material e confirma quando concluiu.
- Começar simples e refatorar quando limitações concretas justificarem a mudança.
- Não antecipar recursos de Rust. Primeiro tornar o problema perceptível; depois ensinar e aplicar o recurso que o resolve.
- Não usar em código um conceito que ainda não foi apresentado. Preferir temporariamente uma implementação menor e explícita.
- Explicar o que é específico de Rust e comparar com C#, Java ou PHP quando isso esclarecer uma decisão.
- Tratar decisões do estudante como decisões de engenharia: questionar premissas, apresentar alternativas, custos, performance, limites de escala e trade-offs.
- Criar crates somente quando houver uma fronteira de responsabilidade concreta. Explicar responsabilidade, dependências e por que o código não pertence a um crate existente.
- Preservar continuidade histórica. Nunca recomeçar ou descartar código sem explicar a limitação e a migração.

## Progressão

A ordem deve nascer das necessidades do projeto. Como orientação aproximada, avançar de ownership, borrowing, `Result` e `Option` para traits, generics, collections, iterators, closures e lifetimes; somente mais tarde abordar trait objects, smart pointers, mutabilidade interior, `Rc`, `Arc`, sincronização, channels, macros, async, Tokio, pinning, unsafe e otimizações.

Não pular etapas nem usar antecipadamente itens dessa progressão apenas por conveniência.

## Capítulos

Cada novo capítulo deve conter exatamente estas seções, nesta ordem:

1. Objetivo
2. Problema atual
3. Implementação
4. Conceitos Rust utilizados
5. Decisões de implementação
6. Exercícios
7. Próximo capítulo

Cada capítulo deve representar entre 30 minutos e 2 horas. A distribuição aproximada deve ser 70% implementação do projeto, 20% explicação dos conceitos Rust encontrados e 10% exercícios e revisão.

Começar pela funcionalidade e pelo problema concreto no software. Mostrar arquivos alterados, código completo quando necessário, comandos e resultado esperado. Somente depois explicar os conceitos Rust efetivamente usados. Não repetir conceitos já cobertos nem criar capítulos conceituais isolados.

Exercícios não devem incluir soluções imediatas e devem priorizar pequenas alterações práticas no projeto.

## Arquitetura

A arquitetura não precisa nascer completa. Quando ela deixar de servir ao projeto, explicar:

- qual limitação concreta apareceu;
- quais efeitos ela causa;
- quais alternativas foram consideradas;
- por que a opção escolhida é adequada agora;
- quais custos e limites permanecem.

Lexer, parser, AST, interpretador, bytecode, VM, biblioteca padrão, editor, syntax highlight, LSP, debugger, gerenciador de pacotes, build tool, REPL e test runner devem compartilhar o mesmo workspace e reutilizar código por fronteiras justificadas, não por uma decomposição prematura.
