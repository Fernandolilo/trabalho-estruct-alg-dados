mod cliente;
mod grafo;
mod produto;

use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

use cliente::Cliente;
use grafo::Grafo;
use produto::Produto;

fn main() {
    // =========================================
    // CADASTRO DOS PRODUTOS
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
    // CADASTRO DOS CLIENTES
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
    // CRIAÇÃO DO GRAFO
    // =========================================

    let mut grafo = Grafo::novo();

    // Clientes
    grafo.adicionar_vertice("Cliente:João".to_string());
    grafo.adicionar_vertice("Cliente:Maria".to_string());

    // Produtos
    grafo.adicionar_vertice("Produto:1".to_string());
    grafo.adicionar_vertice("Produto:2".to_string());
    grafo.adicionar_vertice("Produto:3".to_string());
    grafo.adicionar_vertice("Produto:4".to_string());

    // =========================================
    // RELACIONAMENTOS DO GRAFO
    // =========================================

    // João comprou Notebook
    grafo.adicionar_aresta(
        "Cliente:João".to_string(),
        "Produto:1".to_string(),
    );

    // João comprou Mouse
    grafo.adicionar_aresta(
        "Cliente:João".to_string(),
        "Produto:2".to_string(),
    );

    // Maria comprou Notebook
    grafo.adicionar_aresta(
        "Cliente:Maria".to_string(),
        "Produto:1".to_string(),
    );

    // Notebook relacionado com Teclado
    grafo.adicionar_aresta(
        "Produto:1".to_string(),
        "Produto:3".to_string(),
    );

    // Notebook relacionado com Fone
    grafo.adicionar_aresta(
        "Produto:1".to_string(),
        "Produto:4".to_string(),
    );

    // Mouse relacionado com Teclado
    grafo.adicionar_aresta(
        "Produto:2".to_string(),
        "Produto:3".to_string(),
    );

    // =========================================
    // PRODUTOS COMPRADOS PELO JOÃO
    // =========================================

    let mut produtos_comprados_joao: HashSet<String> = HashSet::new();

    produtos_comprados_joao.insert("Produto:1".to_string());
    produtos_comprados_joao.insert("Produto:2".to_string());

    // =========================================
    // MENU PRINCIPAL
    // =========================================

    loop {
        println!();
        println!("=================================");
        println!("       CONECTASTORE");
        println!("=================================");
        println!("1 - Listar produtos");
        println!("2 - Listar clientes");
        println!("3 - Mostrar grafo");
        println!("4 - Executar BFS");
        println!("5 - Gerar recomendações");
        println!("0 - Sair");
        println!("=================================");

        print!("Escolha uma opção: ");
        io::stdout().flush().unwrap();

        let mut opcao = String::new();

        io::stdin()
            .read_line(&mut opcao)
            .expect("Erro ao ler a opção");

        match opcao.trim() {
            "1" => {
                listar_produtos(&produtos);
            }

            "2" => {
                listar_clientes(&clientes);
            }

            "3" => {
                grafo.mostrar();
            }

            "4" => {
                println!();
                println!("=== PERCURSO BFS ===");

                let percurso = grafo.bfs("Cliente:João");

                for vertice in percurso {
                    println!("{}", vertice);
                }
            }

            "5" => {
                gerar_recomendacoes(
                    &grafo,
                    &produtos,
                    &produtos_comprados_joao,
                );
            }

            "0" => {
                println!();
                println!("Encerrando o ConectaStore...");
                println!("Até logo!");

                break;
            }

            _ => {
                println!();
                println!("❌ Opção inválida!");
                println!("Escolha uma opção entre 0 e 5.");
            }
        }
    }
}

// =========================================
// LISTAR PRODUTOS
// =========================================

fn listar_produtos(produtos: &HashMap<u32, Produto>) {
    println!();
    println!("=== PRODUTOS CADASTRADOS ===");

    for produto in produtos.values() {
        println!(
            "{} - {} ({})",
            produto.id,
            produto.nome,
            produto.categoria
        );
    }
}

// =========================================
// LISTAR CLIENTES
// =========================================

fn listar_clientes(clientes: &HashMap<u32, Cliente>) {
    println!();
    println!("=== CLIENTES CADASTRADOS ===");

    for cliente in clientes.values() {
        println!("{} - {}", cliente.id, cliente.nome);
    }
}

// =========================================
// GERAR RECOMENDAÇÕES
// =========================================

fn gerar_recomendacoes(
    grafo: &Grafo,
    produtos: &HashMap<u32, Produto>,
    produtos_comprados: &HashSet<String>,
) {
    println!();
    println!("=== RECOMENDAÇÕES PARA JOÃO ===");

    let recomendacoes = grafo.recomendar(
        "Cliente:João",
        produtos_comprados,
    );

    if recomendacoes.is_empty() {
        println!("Nenhuma recomendação encontrada.");
        return;
    }

    for (i, (recomendacao, pontuacao)) in recomendacoes.iter().enumerate() {
        let id = recomendacao
            .strip_prefix("Produto:")
            .unwrap()
            .parse::<u32>()
            .unwrap();

        if let Some(produto) = produtos.get(&id) {
            println!();
            println!("{}. 🛒 {}", i + 1, produto.nome);
            println!("   Categoria: {}", produto.categoria);
            println!("   Relevância: {}", pontuacao);
        }
    }
}