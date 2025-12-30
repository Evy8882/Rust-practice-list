# Desafios de Rust (gerado pelo Chat GPT)
## Nível Iniciante ao Intermediário

Este documento reúne uma lista de desafios para praticar Rust, organizados por nível de dificuldade, com descrições claras e sugestões de evolução em cada tarefa.

---

## 🎯 Objetivo

Ajudar você a praticar:

- leitura e escrita no terminal
- controle de fluxo
- tipos e conversões
- ownership e borrowing
- tratamento de erros
- modularização
- collections
- structs e enums
- iteradores

Os desafios foram pensados para evoluir gradualmente sua lógica e domínio da linguagem.

---

## 🟢 Desafios Iniciantes

### 1) Conversor de Temperatura

Faça um programa que:

- leia um valor
- converta entre Celsius ↔ Fahrenheit
- permita escolher o modo
- valide entrada numérica
- trate possíveis erros

Sugestões de evolução:

- mostrar só duas casas decimais
- permitir repetir sem reiniciar o programa
- evitar panic ao parsear entradas inválidas

---

### 2) Calculadora Simples

O programa deve:

- ler dois números
- perguntar a operação: + − × ÷
- exibir o resultado

Requisitos:

- dividir deve tratar divisão por zero
- evitar unwrap direto
- mostrar mensagem de erro quando inválido

Evolução sugerida:

- aceitar operações digitadas por extenso (ex: soma)
- permitir rodar repetidamente
- suportar números negativos

---

### 3) Gerador de Tabuada

Leia um número inteiro e mostre a tabuada de 1 a 10.

Exemplo desejado:

5 x 1 = 5  
5 x 2 = 10  
...  

Extensões possíveis:

- permitir definir o limite
- permitir escolher modo horizontal ou vertical
- gerar tabuada para múltiplos números

---

### 4) Contador Personalizado

O usuário informa:

- início
- fim
- passo

O programa conta respeitando o passo informado.

Desafios extras:

- tratar passo igual a zero
- inverter automaticamente quando necessário
- validar valores digitados

---

## 🟡 Desafios Intermediários

### 5) Jogo de Adivinhação

O programa deve:

- gerar número aleatório
- permitir tentativas do usuário
- dizer se o chute foi maior ou menor
- contar tentativas
- encerrar somente ao acertar

Possíveis melhorias:

- definir intervalo personalizado
- limitar tentativas
- salvar recorde

---

### 6) Lista de Tarefas no Terminal

Funcionalidades esperadas:

- adicionar tarefa
- listar tarefas
- remover tarefa
- marcar como concluída

Sugestão de estrutura:

- usar vetor para armazenar tarefas
- representar tarefa com struct
- usar loop principal de menu

Upgrades:

- salvar em arquivo
- ordenar tarefas
- timestamps

---

### 7) Conversor de Base Numérica

Entrada: um número inteiro

O programa deve mostrar:

- binário
- hexadecimal
- octal

Desafios extras:

- fazer o inverso (converter de binário → decimal)
- validar se a entrada é realmente válida
- aceitar números negativos

---

### 8) Estatísticas de Números

O programa deve:

- ler vários números
- parar com um comando (ex: ENTER vazio)
- ao final exibir:

Resultados desejados:

- quantidade de números
- maior e menor
- média
- somatório

Desafios:

- não usar unwrap
- trabalhar com Option e Result
- permitir floats

---

## 🧠 Dicas de Estudo

Quando resolver um desafio, tente:

1) Refatorar para funções menores  
2) Remover unwraps  
3) Usar match e if let  
4) Explorar ownership e borrowing  
5) Adicionar tratamento de erros personalizado  

---

## 🚀 Próximos Passos

Quando estiver confortável:

- implemente testes unitários
- separe módulos
- use structs e enums
- explore iterators e map/filter
- comece um projeto CLI maior

---
