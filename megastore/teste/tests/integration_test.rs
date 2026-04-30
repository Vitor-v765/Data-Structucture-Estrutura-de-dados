// Importamos as estruturas do nosso projeto
use megastore::{Product, SearchEngine};
use std::sync::{Arc, RwLock};

/// Função auxiliar para configurar o motor com os dados do catálogo
/// Assim não precisamos repetir a inserção em cada teste.
fn setup_engine() -> Arc<RwLock<SearchEngine>> {
    let engine = Arc::new(RwLock::new(SearchEngine::new()));
    let mut write_lock = engine.write().unwrap();
    
    write_lock.add_product(Product {
        id: 1,
        name: "Smartphone Galaxy S23".to_string(),
        brand: "Samsung".to_string(),
        category: "Eletrônicos".to_string(),
        price: 4999.00,
    });

    write_lock.add_product(Product {
        id: 2,
        name: "Smart TV 4K 55 polegadas".to_string(),
        brand: "Samsung".to_string(),
        category: "Eletrônicos".to_string(),
        price: 2500.00,
    });

    write_lock.add_product(Product {
        id: 3,
        name: "Tênis de Corrida Ultraboost".to_string(),
        brand: "Adidas".to_string(),
        category: "Vestuário".to_string(),
        price: 899.90,
    });

    // Liberamos o lock de escrita antes de retornar o motor
    drop(write_lock);
    
    engine
}

#[test]
fn test_busca_por_marca_samsung() {
    let engine = setup_engine();
    let read_lock = engine.read().unwrap();
    
    let results = read_lock.search("Samsung");
    
    // Esperamos encontrar 2 produtos da Samsung
    assert_eq!(results.len(), 2, "Deveria encontrar 2 produtos da Samsung");
}

#[test]
fn test_busca_composta_smartphone_galaxy() {
    let engine = setup_engine();
    let read_lock = engine.read().unwrap();
    
    let results = read_lock.search("smartphone galaxy");
    
    // Esperamos encontrar 1 produto específico
    assert_eq!(results.len(), 1, "Deveria encontrar 1 smartphone galaxy");
    assert_eq!(results[0].id, 1, "O ID do produto deveria ser 1");
}

#[test]
fn test_busca_por_marca_adidas() {
    let engine = setup_engine();
    let read_lock = engine.read().unwrap();
    
    let results = read_lock.search("Adidas");
    
    // Esperamos encontrar 1 produto da Adidas
    assert_eq!(results.len(), 1, "Deveria encontrar 1 produto da Adidas");
    assert_eq!(results[0].category, "Vestuário");
}

#[test]
fn test_busca_produto_inexistente() {
    let engine = setup_engine();
    let read_lock = engine.read().unwrap();
    
    let results = read_lock.search("notebook");
    
    // Esperamos que a lista esteja vazia
    assert!(results.is_empty(), "A busca por 'notebook' deveria retornar vazio");
}