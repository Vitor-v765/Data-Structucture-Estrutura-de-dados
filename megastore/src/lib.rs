use std::collections::{HashMap, HashSet};

/// Representa um produto no catálogo da MegaStore.
#[derive(Debug, Clone)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub brand: String,
    pub category: String,
    pub price: f64,
}

/// O núcleo do Motor de Busca.
pub struct SearchEngine {
    // Armazena os dados reais dos produtos (ID -> Produto)
    products: HashMap<u64, Product>,
    // Índice Invertido: Token (palavra) -> Conjunto de IDs de produtos
    inverted_index: HashMap<String, HashSet<u64>>,
}

impl SearchEngine {
    /// Cria uma nova instância vazia do motor de busca.
    pub fn new() -> Self {
        SearchEngine {
            products: HashMap::new(),
            inverted_index: HashMap::new(),
        }
    }

    /// Função utilitária privada para normalizar e extrair palavras-chave
    fn tokenize(text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            // Remove pontuações básicas para melhorar a precisão
            .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Indexa um produto no catálogo de forma eficiente.
    pub fn add_product(&mut self, product: Product) {
        let product_id = product.id;
        
        // Extrai todos os termos pesquisáveis do produto
        let searchable_text = format!("{} {} {}", product.name, product.brand, product.category);
        let tokens = Self::tokenize(&searchable_text);

        // Insere no armazenamento principal
        self.products.insert(product_id, product);

        // Alimenta o Índice Invertido
        for token in tokens {
            self.inverted_index
                .entry(token)
                .or_insert_with(HashSet::new)
                .insert(product_id);
        }
    }

    /// Realiza a busca rápida combinando os termos pesquisados
    pub fn search(&self, query: &str) -> Vec<Product> {
        let tokens = Self::tokenize(query);
        if tokens.is_empty() {
            return vec![];
        }

        // Busca o conjunto de IDs do primeiro termo da pesquisa
        let mut result_ids = match self.inverted_index.get(&tokens[0]) {
            Some(ids) => ids.clone(),
            None => return vec![], 
        };

        // Interseção com os outros termos da pesquisa
        for token in tokens.iter().skip(1) {
            if let Some(ids) = self.inverted_index.get(token) {
                // Mantém apenas os produtos que contêm TODAS as palavras
                result_ids.retain(|id| ids.contains(id));
            } else {
                return vec![];
            }
        }

        // Retorna os produtos correspondentes
        result_ids
            .iter()
            .filter_map(|id| self.products.get(id))
            .cloned()
            .collect()
    }
}

// Implementação padrão (boa prática em Rust quando temos um método new sem argumentos)
impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}