mod product;
mod search;
mod utils;

use crate::search::SearchEngine;
use crate::product::Product;

fn main() {
    let mut engine = SearchEngine::new();

    engine.add_product(Product::new(1, "Notebook Dell", "Eletrônicos", "Dell"));
    engine.add_product(Product::new(2, "Camisa Polo Azul", "Vestuário", "Lacoste"));
    engine.add_product(Product::new(3, "Cafeteira Elétrica", "Eletrodomésticos", "Philco"));

    let results = engine.search("Dell");

    println!("Resultados encontrados:");
    for product in results {
        println!("- {}", product.name);
    }
}
