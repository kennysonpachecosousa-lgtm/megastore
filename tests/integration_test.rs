use megastore::{
    graph::{Graph, Node},
    models::{Categoria, Cliente, Produto},
    recommendation::recomendar_produtos,
    repository::Repository,
};

use std::time::Instant;

#[test]
fn teste_integracao_completo() {


    // =========================
    // 1. Criar estruturas
    // =========================

    let mut repository = Repository::new();
    let mut graph = Graph::new();

    // =========================
    // 2. Cadastrar categoria
    // =========================

    let categoria = Categoria {
        id: 1,
        nome: "Informática".to_string(),
    };

    repository.cadastrar_categoria(categoria);

    // =========================
    // 3. Cadastrar cliente
    // =========================

    let cliente = Cliente {
        id: 1,
        nome: "Kennyson".to_string(),
    };

    repository.cadastrar_cliente(cliente);

    // =========================
    // 4. Cadastrar produtos
    // =========================

    let produto_101 = Produto {
        id: 101,
        nome: "Notebook".to_string(),
        categoria_id: 1,
        preco: 100.0,
    };

    let produto_102 = Produto {
        id: 102,
        nome: "Mouse".to_string(),
        categoria_id: 1,
        preco: 50.0,
    };

    let produto_103 = Produto {
        id: 103,
        nome: "Headset".to_string(),
        categoria_id: 1,
        preco: 75.0,
    };

    repository.cadastrar_produto(produto_101);
    repository.cadastrar_produto(produto_102);
    repository.cadastrar_produto(produto_103);

    // =========================
    // 5. Criar vértices
    // =========================

    graph.add_node(Node::Cliente(1));
    graph.add_node(Node::Produto(101));
    graph.add_node(Node::Produto(102));
    graph.add_node(Node::Produto(103));

    graph.add_edge(
        Node::Cliente(1),
        Node::Produto(101),
        1.0,
        "compra".to_string(),
    );

    graph.add_edge(
        Node::Produto(101),
        Node::Produto(102),
        0.8,
        "similaridade".to_string(),
    );

    graph.add_edge(
        Node::Produto(101),
        Node::Produto(103),
        0.7,
        "similaridade".to_string(),
    );

    // =========================
    // 7. Gerar recomendações
    // =========================

    let recomendacoes = recomendar_produtos(&graph, 1, 2);

    // =========================
    // 8. Validar recomendações
    // =========================

    assert_eq!(recomendacoes[0].0, 102);
    assert_eq!(recomendacoes[0].1, 0.8);

    assert_eq!(recomendacoes[1].0, 103);
    assert_eq!(recomendacoes[1].1, 0.7);

    // O próprio produto 101 não deve ser recomendado
    assert!(!recomendacoes.iter().any(|(id, _)| *id == 101));

    // Os produtos recomendados precisam existir no repository
    assert!(repository.buscar_produto(102).is_some());
    assert!(repository.buscar_produto(103).is_some());
}
#[test]
fn teste_recomendacao_sem_produtos_comprados() {
    let mut graph = Graph::new();

    // Cliente sem nenhum produto comprado
    graph.add_node(Node::Cliente(2));

    let recomendacoes = recomendar_produtos(&graph, 2, 2);

    // Não deve haver recomendações
    assert!(recomendacoes.is_empty());
}

#[test]
fn teste_recomendacao_sem_candidatos() {
    let mut graph = Graph::new();

    // Cliente e produto
    graph.add_node(Node::Cliente(3));
    graph.add_node(Node::Produto(201));

    // Cliente comprou o único produto existente
    graph.add_edge(
        Node::Cliente(3),
        Node::Produto(201),
        1.0,
        "compra".to_string(),
    );

    let recomendacoes = recomendar_produtos(&graph, 3, 2);

    // Como não existe outro produto para recomendar,
    // o resultado deve ser vazio.
    assert!(recomendacoes.is_empty());
}
#[test]
fn teste_desempenho_recomendacao() {
    let tamanhos = [100, 500, 1000];

    for tamanho in tamanhos {
        let mut graph = Graph::new();

        graph.add_node(Node::Cliente(1));

        for id in 1..=tamanho {
            graph.add_node(Node::Produto(id));
        }

        graph.add_edge(
            Node::Cliente(1),
            Node::Produto(1),
            1.0,
            "compra".to_string(),
        );

        for id in 1..tamanho {
            graph.add_edge(
                Node::Produto(id),
                Node::Produto(id + 1),
                0.8,
                "similaridade".to_string(),
            );
        }

        let inicio = Instant::now();

        let recomendacoes = recomendar_produtos(&graph, 1, 2);

        let duracao = inicio.elapsed();

        println!(
            "Produtos: {} | Recomendações: {} | Tempo: {:?}",
            tamanho,
            recomendacoes.len(),
            duracao
        );
    }
}