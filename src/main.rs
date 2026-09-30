use megastore::graph::{Graph, Node};
use megastore::models::{Categoria, Cliente, Produto};
use megastore::recommendation::recomendar_produtos;
use megastore::repository::Repository;

fn main() {
    // Criando o repositório e o grafo
    let mut repository = Repository::new();
    let mut graph = Graph::new();

    // =========================
    // CADASTRO DE CATEGORIAS
    // =========================

    repository.cadastrar_categoria(Categoria {
        id: 1,
        nome: "Eletrônicos".to_string(),
    });

    repository.cadastrar_categoria(Categoria {
        id: 2,
        nome: "Informática".to_string(),
    });

    // =========================
    // CADASTRO DE CLIENTES
    // =========================

    repository.cadastrar_cliente(Cliente {
        id: 1,
        nome: "Carlos".to_string(),
    });

    // =========================
    // CADASTRO DE PRODUTOS
    // =========================

    repository.cadastrar_produto(Produto {
        id: 101,
        nome: "Notebook".to_string(),
        categoria_id: 2,
        preco: 3500.00,
    });

    repository.cadastrar_produto(Produto {
        id: 102,
        nome: "Mouse".to_string(),
        categoria_id: 2,
        preco: 80.00,
    });

    repository.cadastrar_produto(Produto {
        id: 103,
        nome: "Teclado".to_string(),
        categoria_id: 2,
        preco: 150.00,
    });

    repository.cadastrar_produto(Produto {
        id: 104,
        nome: "Headset".to_string(),
        categoria_id: 1,
        preco: 220.00,
    });

    // =========================
    // ADICIONANDO VÉRTICES
    // =========================

    graph.add_node(Node::Cliente(1));

    graph.add_node(Node::Produto(101));
    graph.add_node(Node::Produto(102));
    graph.add_node(Node::Produto(103));
    graph.add_node(Node::Produto(104));

    graph.add_node(Node::Categoria(1));
    graph.add_node(Node::Categoria(2));

    // =========================
    // RELAÇÕES DE COMPRA
    // =========================

    graph.add_edge(
        Node::Cliente(1),
        Node::Produto(101),
        1.0,
        "compra".to_string(),
    );

    graph.add_edge(
        Node::Cliente(1),
        Node::Produto(103),
        1.0,
        "compra".to_string(),
    );

    // =========================
    // RELAÇÕES DE SIMILARIDADE
    // =========================

    graph.add_edge(
        Node::Produto(101),
        Node::Produto(102),
        0.8,
        "similaridade".to_string(),
    );

    graph.add_edge(
        Node::Produto(101),
        Node::Produto(104),
        0.7,
        "similaridade".to_string(),
    );

    // =========================
    // RELAÇÃO COM CATEGORIAS
    // =========================

    graph.add_edge(
        Node::Produto(101),
        Node::Categoria(2),
        1.0,
        "categoria".to_string(),
    );

    graph.add_edge(
        Node::Produto(102),
        Node::Categoria(2),
        1.0,
        "categoria".to_string(),
    );

    graph.add_edge(
        Node::Produto(103),
        Node::Categoria(2),
        1.0,
        "categoria".to_string(),
    );

    graph.add_edge(
        Node::Produto(104),
        Node::Categoria(1),
        1.0,
        "categoria".to_string(),
    );

    // =========================
    // SISTEMA
    // =========================

    println!("=================================");
    println!("       CONECTASTORE");
    println!("=================================");

    // Consulta de produto
    println!("\n--- Consulta de produto ---");

    if let Some(produto) = repository.buscar_produto(101) {
        println!("ID: {}", produto.id);
        println!("Nome: {}", produto.nome);
        println!("Preço: R$ {:.2}", produto.preco);
    }

    // Consulta de cliente
    println!("\n--- Consulta de cliente ---");

    if let Some(cliente) = repository.buscar_cliente(1) {
        println!("ID: {}", cliente.id);
        println!("Nome: {}", cliente.nome);
    }

    // Recomendações
    println!("\n--- Recomendações ---");

    let recomendacoes = recomendar_produtos(&graph, 1, 5);

    if recomendacoes.is_empty() {
        println!("Nenhuma recomendação encontrada.");
    } else {
        for (posicao, (produto_id, relevancia)) in recomendacoes.iter().enumerate() {
            if let Some(produto) = repository.buscar_produto(*produto_id) {
                println!(
                    "{}. {} | R$ {:.2} | Relevância: {:.2}",
                    posicao + 1,
                    produto.nome,
                    produto.preco,
                    relevancia
                );
            }
        }
    }

    println!("\n=================================");
    println!("Sistema finalizado.");
    println!("=================================");
}
