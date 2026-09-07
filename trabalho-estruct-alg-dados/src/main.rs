use std::collections::HashMap;

#[derive(Debug)]
struct Produto {
    id: u32,
    nome: String,
    categoria: String,
}

#[derive(Debug)]
struct Cliente {
    id: u32,
    nome: String,
}

fn main() {
    println!("=== ConectaStore ===");
    println!("Sistema de recomendação de produtos\n");

    // =========================
    // PRODUTOS
    // =========================

    let mut produtos: HashMap<u32, Produto> = HashMap::new();

    produtos.insert(
        1,
        Produto {
            id: 1,
            nome: String::from("Notebook"),
            categoria: String::from("Eletrônicos"),
        },
    );

    produtos.insert(
        2,
        Produto {
            id: 2,
            nome: String::from("Mouse"),
            categoria: String::from("Eletrônicos"),
        },
    );

    produtos.insert(
        3,
        Produto {
            id: 3,
            nome: String::from("Teclado"),
            categoria: String::from("Eletrônicos"),
        },
    );

    // =========================
    // CLIENTES
    // =========================

    let mut clientes: HashMap<u32, Cliente> = HashMap::new();

    clientes.insert(
        1,
        Cliente {
            id: 1,
            nome: String::from("João"),
        },
    );

    clientes.insert(
        2,
        Cliente {
            id: 2,
            nome: String::from("Maria"),
        },
    );

    // =========================
    // EXIBIÇÃO
    // =========================

    println!("Produtos cadastrados:");

    for produto in produtos.values() {
        println!(
            "ID: {} | {} | Categoria: {}",
            produto.id,
            produto.nome,
            produto.categoria
        );
    }

    println!("\nClientes cadastrados:");

    for cliente in clientes.values() {
        println!(
            "ID: {} | Nome: {}",
            cliente.id,
            cliente.nome
        );
    }
}