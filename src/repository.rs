use std::collections::HashMap;

use crate::models::{Categoria, Cliente, Produto};

pub struct Repository {
    pub produtos: HashMap<u32, Produto>,
    pub clientes: HashMap<u32, Cliente>,
    pub categorias: HashMap<u32, Categoria>,
}

impl Repository {
    pub fn new() -> Self {
        Self {
            produtos: HashMap::new(),
            clientes: HashMap::new(),
            categorias: HashMap::new(),
        }
    }

    pub fn cadastrar_produto(&mut self, produto: Produto) {
        self.produtos.insert(produto.id, produto);
    }

    pub fn cadastrar_cliente(&mut self, cliente: Cliente) {
        self.clientes.insert(cliente.id, cliente);
    }

    pub fn cadastrar_categoria(&mut self, categoria: Categoria) {
        self.categorias.insert(categoria.id, categoria);
    }

    pub fn buscar_produto(&self, id: u32) -> Option<&Produto> {
        self.produtos.get(&id)
    }

    pub fn buscar_cliente(&self, id: u32) -> Option<&Cliente> {
        self.clientes.get(&id)
    }

    pub fn buscar_categoria(&self, id: u32) -> Option<&Categoria> {
        self.categorias.get(&id)
    }
}

impl Default for Repository {
    fn default() -> Self {
        Self::new()
    }
}
