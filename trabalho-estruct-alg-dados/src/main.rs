mod cliente;
mod grafo;
mod produto;

use std::collections::{HashMap, HashSet};

use cliente::Cliente;
use grafo::Grafo;
use produto::Produto;

fn main() {
    println!("=================================");
    println!("       CONECTASTORE");
    println!("=================================");

    // =========================================
    // CADASTRO DE PRODUTOS
    // =========================================

    let mut produtos: HashMap<u32, Produto> = HashMap::new();

    produtos.insert(
        1,
        Produto {
            id: 1,
            nome: "Notebook".to_string(),
            categoria: "Eletrônicos".to_string(),
        },
    );

    produtos.insert(
        2,
        Produto {
            id: 2,
            nome: "Mouse".to_string(),
            categoria: "Eletrônicos".to_string(),
        },
    );

    produtos.insert(
        3,
        Produto {
            id: 3,
            nome: "Teclado".to_string(),
            categoria: "Eletrônicos".to_string(),
        },
    );

    produtos.insert(
        4,
        Produto {
            id: 4,
            nome: "Fone de Ouvido".to_string(),
            categoria: "Eletrônicos".to_string(),
        },
    );

    // =========================================
    // CADASTRO DE CLIENTES
    // =========================================

    let mut clientes: HashMap<u32, Cliente> = HashMap::new();

    clientes.insert(
        1,
        Cliente {
            id: 1,
            nome: "João".to_string(),
        },
    );

    clientes.insert(
        2,
        Cliente {
            id: 2,
            nome: "Maria".to_string(),
        },
    );

    // =========================================
    // EXIBIR PRODUTOS
    // =========================================

    println!("\nProdutos cadastrados:");

    for produto in produtos.values() {
        println!(
            "{} - {} ({})",
            produto.id,
            produto.nome,
            produto.categoria
        );
    }

    // =========================================
    // EXIBIR CLIENTES
    // =========================================

    println!("\nClientes cadastrados:");

    for cliente in clientes.values() {
        println!("{} - {}", cliente.id, cliente.nome);
    }

    // =========================================
    // CRIAR GRAFO
    // =========================================

    let mut grafo = Grafo::novo();

    // =========================================
    // ADICIONAR VÉRTICES
    // =========================================

    grafo.adicionar_vertice("Cliente:João".to_string());
    grafo.adicionar_vertice("Cliente:Maria".to_string());

    grafo.adicionar_vertice("Produto:Notebook".to_string());
    grafo.adicionar_vertice("Produto:Mouse".to_string());
    grafo.adicionar_vertice("Produto:Teclado".to_string());
    grafo.adicionar_vertice("Produto:Fone".to_string());

    // =========================================
    // RELACIONAMENTOS DOS CLIENTES
    // =========================================

    // João comprou Notebook
    grafo.adicionar_aresta(
        "Cliente:João".to_string(),
        "Produto:Notebook".to_string(),
    );

    // João comprou Mouse
    grafo.adicionar_aresta(
        "Cliente:João".to_string(),
        "Produto:Mouse".to_string(),
    );

    // Maria comprou Notebook
    grafo.adicionar_aresta(
        "Cliente:Maria".to_string(),
        "Produto:Notebook".to_string(),
    );

    // =========================================
    // SIMILARIDADE ENTRE PRODUTOS
    // =========================================

    // Notebook relacionado com Teclado
    grafo.adicionar_aresta(
        "Produto:Notebook".to_string(),
        "Produto:Teclado".to_string(),
    );

    // Notebook relacionado com Fone
    grafo.adicionar_aresta(
        "Produto:Notebook".to_string(),
        "Produto:Fone".to_string(),
    );

    // Mouse relacionado com Teclado
    grafo.adicionar_aresta(
        "Produto:Mouse".to_string(),
        "Produto:Teclado".to_string(),
    );

    // =========================================
    // MOSTRAR GRAFO
    // =========================================

    grafo.mostrar();

    // =========================================
    // PERCURSO BFS
    // =========================================

    println!("\n=== PERCURSO BFS ===");

    let percurso = grafo.bfs("Cliente:João");

    for vertice in &percurso {
        println!("{}", vertice);
    }

    // =========================================
    // PRODUTOS JÁ COMPRADOS POR JOÃO
    // =========================================

    let mut produtos_comprados: HashSet<String> = HashSet::new();

    produtos_comprados.insert("Produto:Notebook".to_string());
    produtos_comprados.insert("Produto:Mouse".to_string());

    // =========================================
    // RECOMENDAÇÕES
    // =========================================

    println!("\n=== RECOMENDAÇÕES PARA JOÃO ===");

    let recomendacoes = grafo.recomendar(
        "Cliente:João",
        &produtos_comprados,
    );

    for (i, (recomendacao, pontuacao)) in recomendacoes.iter().enumerate() {
        println!(
            "{}. 🛒 {} - Relevância: {}",
            i + 1,
            recomendacao,
            pontuacao
        );
    }
}