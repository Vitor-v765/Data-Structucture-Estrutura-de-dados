// Importamos as estruturas do nosso arquivo lib.rs
// O nome 'megastore' assume que você rodou `cargo new megastore`
use megastore::{Product, SearchEngine};
use std::sync::{Arc, RwLock};

fn main() {
    println!("Inicializando o Motor de Busca da MegaStore...\n");

    // Envolvemos a SearchEngine em Arc e RwLock para uso seguro em threads 
    // (preparando para alta escala em um servidor web)
    let engine = Arc::new(RwLock::new(SearchEngine::new()));

    // --- FASE DE INDEXAÇÃO ---
    {
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
    }

    // --- FASE DE BUSCA ---
    let read_lock = engine.read().unwrap();

    // Lista de pesquisas para simular clientes na loja
    let queries = ["Samsung", "smartphone galaxy", "Adidas", "notebook"];

    for query in queries {
        println!("--- Buscando por '{}': ---", query);
        let results = read_lock.search(query);
        
        if results.is_empty() {
            println!("Nenhum produto encontrado.");
        } else {
            for p in results {
                println!("- {} (R$ {:.2}) [Categoria: {}]", p.name, p.price, p.category);
            }
        }
        println!();
    }
}