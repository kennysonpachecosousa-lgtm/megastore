# ConectaStore — Sistema de Recomendação de Produtos Baseado em Grafos

## 1. Sobre o projeto

O ConectaStore é um sistema de recomendação de produtos desenvolvido em Rust para o cenário fictício da MegaStore.

O sistema utiliza uma estrutura de grafo para representar relações entre clientes e produtos, permitindo identificar produtos relacionados e gerar recomendações com base nas conexões existentes.

O projeto foi desenvolvido como parte da disciplina de Estruturas de Dados e tem como objetivo demonstrar a aplicação prática de grafos, estruturas de dados, algoritmos de recomendação, testes automatizados e análise básica de desempenho.

## 2. Objetivos

- Cadastrar e consultar produtos.
- Representar clientes e produtos em um grafo.
- Criar conexões entre clientes e produtos.
- Representar relações de similaridade entre produtos.
- Gerar recomendações de produtos.
- Evitar recomendações de produtos já comprados.
- Utilizar estruturas como HashMap.
- Realizar testes automatizados.
- Medir o tempo de execução com diferentes quantidades de produtos.

## 3. Tecnologias utilizadas

- Rust
- Cargo
- Git e GitHub
- WSL 2
- Ubuntu 24.04 LTS

## 4. Estruturas de dados

O projeto utiliza principalmente:

### HashMap

O HashMap é utilizado para armazenar e consultar informações de forma eficiente, utilizando identificadores como chave.

### Grafo

O grafo representa as relações existentes no sistema.

Os principais tipos de nós são:

- Cliente
- Produto

As arestas representam relações entre os nós, como:

- `compra`
- `similaridade`

As arestas também possuem pesos, representando a intensidade ou relevância da relação.

## 5. Modelo do grafo

Exemplo simplificado:

```text
Cliente 1
    |
   compra
    |
    v
Produto 101
    |
    | similaridade 0.8
    v
Produto 102
    |
    | similaridade 0.7
    v
Produto 103
