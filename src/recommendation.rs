use std::collections::{HashMap, HashSet};

use crate::graph::{Graph, Node};

pub fn recomendar_produtos(graph: &Graph, cliente_id: u32, limite: usize) -> Vec<(u32, f64)> {
    let cliente = Node::Cliente(cliente_id);

    // ============================================================
    // 1. Descobrir os produtos que o cliente já comprou
    // ============================================================

    let mut produtos_comprados: HashSet<u32> = HashSet::new();

    if let Some(conexoes) = graph.adjacency.get(&cliente) {
        for aresta in conexoes {
            if aresta.tipo == "compra"
                && let Node::Produto(produto_id) = &aresta.destino
            {
                produtos_comprados.insert(*produto_id);
            }
        }
    }

    // Se o cliente não comprou nenhum produto,
    // não existem recomendações para gerar.
    if produtos_comprados.is_empty() {
        return Vec::new();
    }

    // ============================================================
    // 2. Armazenar a maior relevância encontrada para cada produto
    // ============================================================

    let mut candidatos: HashMap<u32, f64> = HashMap::new();

    // ============================================================
    // 3. Procurar produtos semelhantes aos produtos comprados
    // ============================================================

    for produto_comprado in &produtos_comprados {
        let produto = Node::Produto(*produto_comprado);

        if let Some(conexoes) = graph.adjacency.get(&produto) {
            for aresta in conexoes {
                // Considera somente relações de similaridade.
                if aresta.tipo != "similaridade" {
                    continue;
                }

                // A recomendação precisa apontar para um produto.
                let produto_id = match &aresta.destino {
                    Node::Produto(id) => *id,
                    _ => continue,
                };

                // Não recomenda produtos que o cliente já comprou.
                if produtos_comprados.contains(&produto_id) {
                    continue;
                }

                // Guarda o maior peso encontrado para esse produto.
                let entrada = candidatos.entry(produto_id).or_insert(0.0);

                if aresta.peso > *entrada {
                    *entrada = aresta.peso;
                }
            }
        }
    }

    // ============================================================
    // 4. Transformar os candidatos em vetor
    // ============================================================

    println!("Produtos comprados: {:?}", produtos_comprados);
    println!("Candidatos: {:?}", candidatos);

    let mut recomendacoes: Vec<(u32, f64)> = candidatos.into_iter().collect();

    // ============================================================
    // 5. Ordenar pela maior relevância
    // ============================================================

    recomendacoes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // ============================================================
    // 6. Limitar a quantidade de recomendações
    // ============================================================

    recomendacoes.truncate(limite);

    // ============================================================
    // 7. Retornar as recomendações
    // ============================================================

    recomendacoes
}
