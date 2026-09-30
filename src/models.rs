#[derive(Debug, Clone)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub categoria_id: u32,
    pub preco: f64,
}

#[derive(Debug, Clone)]
pub struct Cliente {
    pub id: u32,
    pub nome: String,
}

#[derive(Debug, Clone)]
pub struct Categoria {
    pub id: u32,
    pub nome: String,
}
