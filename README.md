# 🛒 Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## 📌 Descrição
Este projeto implementa um **sistema de busca otimizado** para o catálogo de produtos da MegaStore, utilizando **estruturas de dados baseadas em tabelas hash** para garantir buscas rápidas e escaláveis, desenvolvido em **Rust**.

## ⚙️ Funcionalidades
- Indexação de produtos por ID, nome, categoria e marca.  
- Busca paralela otimizada com `rayon`.  
- Estrutura de dados eficiente usando `hashbrown::HashMap`.  
- Testes unitários automatizados.

## 🧠 Tecnologias
- Linguagem: **Rust**
- Crates: `serde`, `serde_json`, `hashbrown`, `rayon`
- Testes: `cargo test`

## 🚀 Execução

### 1️⃣ Clonar o repositório
```bash
git clone https://github.com/seu-usuario/megastore-search.git
cd megastore-search
```

### 2️⃣ Executar o projeto
```bash
cargo run
```

### 3️⃣ Rodar os testes
```bash
cargo test
```

## 🧩 Estrutura do Projeto
```
src/
├── product.rs
├── search.rs
├── utils.rs
├── main.rs
tests/
└── search_tests.rs
```

## 📈 Considerações sobre desempenho
- A busca é executada em paralelo, reduzindo o tempo de resposta em catálogos grandes.  
- O uso de `HashMap` garante **complexidade média O(1)** nas inserções e consultas.  
- O sistema pode ser expandido para suportar filtros por preço, relevância e recomendação.

## 🧾 Licença
Distribuído sob a licença **MIT**.
