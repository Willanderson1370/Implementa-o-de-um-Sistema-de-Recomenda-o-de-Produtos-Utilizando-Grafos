use crate::product::Product;
use hashbrown::HashMap;
use rayon::prelude::*;

pub struct SearchEngine {
    index: HashMap<u32, Product>,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.index.insert(product.id, product);
    }

    pub fn search(&self, query: &str) -> Vec<&Product> {
        let q = query.to_lowercase();

        self.index
            .par_iter()
            .filter_map(|(_, p)| {
                if p.name.to_lowercase().contains(&q)
                    || p.brand.to_lowercase().contains(&q)
                    || p.category.to_lowercase().contains(&q)
                {
                    Some(p)
                } else {
                    None
                }
            })
            .collect()
    }
}
