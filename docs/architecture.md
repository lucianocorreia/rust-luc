# Arquitetura

## Estado oficial

O estado oficial implementado corresponde ao fim do Capítulo 1.

```text
Cargo Workspace virtual
└── package luc
    └── crate binário luc
        └── main
```

O binário apenas imprime `Luc`. Os capítulos 2 a 11 são guias ainda não implementados pelo estudante.

## Decisões atuais

### Workspace virtual

A raiz coordena packages sem produzir artefato próprio. Isso permite crescimento futuro sem fingir que já existem fronteiras maduras.

### Um único package

Não existe ainda lógica reutilizada por dois consumidores, responsabilidade independente ou necessidade de compilação isolada. Separar lexer, parser ou runtime agora seria decomposição por previsão.

### Binário como primeira fronteira

O package `luc` produz o comando que servirá de entrada inicial para o ecossistema. Biblioteca, REPL e serviços de linguagem surgirão somente quando houver código concreto para compartilhar.

## Primeira refatoração planejada

O Capítulo 6 propõe extrair uma função privada `run` depois que `main` acumular protocolo CLI e leitura do fonte.

A mudança ainda permanece dentro do crate porque uma função resolve a pressão observada. Criar outro crate nesse ponto acrescentaria API pública, dependência e coordenação sem segundo consumidor.

## Primeiro módulo planejado

O Capítulo 7 propõe `lexer.rs` quando a análise léxica se torna uma responsabilidade concreta. Ele permanece no package `luc`: existe separação de código, mas ainda não existe segundo consumidor que justifique uma fronteira de crate.

Os capítulos 8 a 11 evoluem esse mesmo módulo com operadores, identificadores, números, strings e erros léxicos. Nenhuma dessas etapas cria package adicional.

## Registro de decisões

Este documento descreve apenas decisões já implementadas ou refatorações apresentadas nos guias disponíveis. Atualize-o depois de concluir cada capítulo, nunca antes de confirmar a implementação.
