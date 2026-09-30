# ConectaStore — Sistema de Recomendação de Produtos Baseado em Grafos

## 1. Sobre o projeto

O ConectaStore é um sistema de recomendação de produtos desenvolvido em Rust para o cenário fictício da MegaStore.

O sistema utiliza um grafo para representar relações entre clientes e produtos. A partir das conexões existentes entre produtos, o sistema consegue identificar itens relacionados e gerar recomendações considerando o peso das relações de similaridade.

O projeto foi desenvolvido para demonstrar, na prática, a utilização de estruturas de dados, grafos, algoritmos de busca, estruturas HashMap e HashSet, testes automatizados e análise básica de desempenho.

---

## 2. Objetivos

O projeto possui os seguintes objetivos:

- Cadastrar e consultar produtos.
- Representar clientes e produtos em um grafo.
- Criar conexões entre clientes e produtos.
- Representar relações de similaridade entre produtos.
- Associar pesos às relações de similaridade.
- Gerar recomendações de produtos.
- Evitar recomendar produtos que o cliente já comprou.
- Utilizar estruturas eficientes como HashMap e HashSet.
- Implementar busca em largura (BFS).
- Realizar testes unitários e de integração.
- Medir o desempenho com diferentes quantidades de produtos.

---

## 3. Tecnologias utilizadas

- Rust
- Cargo
- Git
- GitHub
- WSL 2
- Ubuntu 24.04 LTS

---

## 4. Arquitetura do projeto

O projeto está organizado da seguinte forma:

```text
megastore/
├── src/
│   ├── graph.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── models.rs
│   ├── recommendation.rs
│   └── repository.rs
│
├── tests/
│   └── integration_test.rs
│
├── Cargo.toml
├── Cargo.lock
├── README.md
└── .gitignore

---

## 5. Pitch

Vídeo de apresentação e demonstração do projeto:

https://youtu.be/pgjc9Sdr_g8