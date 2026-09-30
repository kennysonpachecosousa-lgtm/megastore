use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Cliente(u32),
    Produto(u32),
    Categoria(u32),
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub destino: Node,
    pub peso: f64,
    pub tipo: String,
}

pub struct Graph {
    pub adjacency: HashMap<Node, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            adjacency: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.adjacency.entry(node).or_default();
    }

    pub fn add_edge(&mut self, origem: Node, destino: Node, peso: f64, tipo: String) {
        self.adjacency.entry(origem).or_default().push(Edge {
            destino,
            peso,
            tipo,
        });
    }

    pub fn bfs(&self, inicio: &Node) -> Vec<Node> {
        let mut visitados = HashSet::new();
        let mut fila = VecDeque::new();
        let mut resultado = Vec::new();

        fila.push_back(inicio.clone());
        visitados.insert(inicio.clone());

        while let Some(atual) = fila.pop_front() {
            resultado.push(atual.clone());

            if let Some(vizinhos) = self.adjacency.get(&atual) {
                for aresta in vizinhos {
                    if !visitados.contains(&aresta.destino) {
                        visitados.insert(aresta.destino.clone());
                        fila.push_back(aresta.destino.clone());
                    }
                }
            }
        }

        resultado
    }
}
impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deve_adicionar_vertice() {
        let mut graph = Graph::new();

        graph.add_node(Node::Produto(1));

        assert!(graph.adjacency.contains_key(&Node::Produto(1)));
    }

    #[test]
    fn deve_adicionar_aresta() {
        let mut graph = Graph::new();

        graph.add_edge(
            Node::Cliente(1),
            Node::Produto(10),
            1.0,
            "compra".to_string(),
        );

        let conexoes = graph.adjacency.get(&Node::Cliente(1)).unwrap();

        assert_eq!(conexoes.len(), 1);
        assert_eq!(conexoes[0].destino, Node::Produto(10));
        assert_eq!(conexoes[0].tipo, "compra");
    }

    #[test]
    fn bfs_deve_percorrer_o_grafo() {
        let mut graph = Graph::new();

        graph.add_edge(
            Node::Cliente(1),
            Node::Produto(10),
            1.0,
            "compra".to_string(),
        );

        graph.add_edge(
            Node::Produto(10),
            Node::Produto(20),
            0.8,
            "similaridade".to_string(),
        );

        let resultado = graph.bfs(&Node::Cliente(1));

        assert!(resultado.contains(&Node::Cliente(1)));
        assert!(resultado.contains(&Node::Produto(10)));
        assert!(resultado.contains(&Node::Produto(20)));
    }
}
