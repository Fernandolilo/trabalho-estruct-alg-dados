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
    // CONTROLE DE COMPRAS
    // =========================================
    //
    // HashMap:
    //
    // ID do cliente -> HashSet de produtos
    //
    // João -> Notebook, Mouse
    // Maria -> Notebook
    //
    // =========================================

    let mut compras: HashMap<u32, HashSet<u32>> = HashMap::new();

    // Compras iniciais do João
    compras
        .entry(1)
        .or_insert_with(HashSet::new)
        .insert(1);

    compras
        .entry(1)
        .or_insert_with(HashSet::new)
        .insert(2);

    // Compra inicial da Maria
    compras
        .entry(2)
        .or_insert_with(HashSet::new)
        .insert(1);

    // =========================================
    // CRIAÇÃO DO GRAFO
    // =========================================

    let mut grafo = Grafo::novo();

    // =========================================
    // VÉRTICES DOS CLIENTES
    // =========================================

    for cliente in clientes.values() {
        grafo.adicionar_vertice(format!("Cliente:{}", cliente.nome));
    }

    // =========================================
    // VÉRTICES DOS PRODUTOS
    // =========================================

    for produto in produtos.values() {
        grafo.adicionar_vertice(format!("Produto:{}", produto.id));
    }

    // =========================================
    // RELACIONAMENTOS CLIENTE -> PRODUTO
    // =========================================

    // João -> Notebook
    grafo.adicionar_aresta(
        "Cliente:João".to_string(),
        "Produto:1".to_string(),
    );

    // João -> Mouse
    grafo.adicionar_aresta(
        "Cliente:João".to_string(),
        "Produto:2".to_string(),
    );

    // Maria -> Notebook
    grafo.adicionar_aresta(
        "Cliente:Maria".to_string(),
        "Produto:1".to_string(),
    );

    // =========================================
    // RELACIONAMENTOS ENTRE PRODUTOS
    // =========================================

    // Notebook -> Teclado
    grafo.adicionar_aresta(
        "Produto:1".to_string(),
        "Produto:3".to_string(),
    );

    // Notebook -> Fone
    grafo.adicionar_aresta(
        "Produto:1".to_string(),
        "Produto:4".to_string(),
    );

    // Mouse -> Teclado
    grafo.adicionar_aresta(
        "Produto:2".to_string(),
        "Produto:3".to_string(),
    );

    // =========================================
    // PRODUTOS COMPRADOS PELO JOÃO
    // =========================================
    //
    // Utilizado pela função atual de recomendação.
    //
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
        println!("          CONECTASTORE");
        println!("=================================");
        println!("1 - Listar produtos");
        println!("2 - Listar clientes");
        println!("3 - Mostrar grafo");
        println!("4 - Executar BFS");
        println!("5 - Gerar recomendações");
        println!("6 - Registrar compra");
        println!("7 - Mostrar compras");
        println!("0 - Sair");
        println!("=================================");

        print!("Escolha uma opção: ");
        io::stdout().flush().unwrap();

        let mut opcao = String::new();

        io::stdin()
            .read_line(&mut opcao)
            .expect("Erro ao ler a opção");

        match opcao.trim() {
            // =================================
            // 1 - LISTAR PRODUTOS
            // =================================

            "1" => {
                listar_produtos(&produtos);
            }

            // =================================
            // 2 - LISTAR CLIENTES
            // =================================

            "2" => {
                listar_clientes(&clientes);
            }

            // =================================
            // 3 - MOSTRAR GRAFO
            // =================================

            "3" => {
                grafo.mostrar();
            }

            // =================================
            // 4 - EXECUTAR BFS
            // =================================

            "4" => {
                println!();
                println!("=================================");
                println!("          PERCURSO BFS");
                println!("=================================");

                let percurso = grafo.bfs("Cliente:João");

                if percurso.is_empty() {
                    println!("Nenhum vértice encontrado.");
                } else {
                    for (posicao, vertice) in percurso.iter().enumerate() {
                        println!("{} - {}", posicao + 1, vertice);
                    }
                }
            }

            // =================================
            // 5 - RECOMENDAÇÕES
            // =================================

            "5" => {
                gerar_recomendacoes(
                    &grafo,
                    &produtos,
                    &produtos_comprados_joao,
                );
            }

            // =================================
            // 6 - REGISTRAR COMPRA
            // =================================

            "6" => {
                registrar_compra(
                    &clientes,
                    &produtos,
                    &mut compras,
                    &mut grafo,
                );
            }

            // =================================
            // 7 - MOSTRAR COMPRAS
            // =================================

            "7" => {
                mostrar_compras(
                    &clientes,
                    &produtos,
                    &compras,
                );
            }

            // =================================
            // 0 - SAIR
            // =================================

            "0" => {
                println!();
                println!("=================================");
                println!("Encerrando o ConectaStore...");
                println!("Até logo!");
                println!("=================================");

                break;
            }

            // =================================
            // OPÇÃO INVÁLIDA
            // =================================

            _ => {
                println!();
                println!("❌ Opção inválida!");
                println!("Escolha uma opção entre 0 e 7.");
            }
        }
    }
}

// =========================================
// LISTAR PRODUTOS
// =========================================

fn listar_produtos(produtos: &HashMap<u32, Produto>) {
    println!();
    println!("=================================");
    println!("       PRODUTOS CADASTRADOS");
    println!("=================================");

    for produto in produtos.values() {
        println!(
            "{} - {} ({})",
            produto.id,
            produto.nome,
            produto.categoria
        );
    }

    println!("=================================");
}

// =========================================
// LISTAR CLIENTES
// =========================================

fn listar_clientes(clientes: &HashMap<u32, Cliente>) {
    println!();
    println!("=================================");
    println!("       CLIENTES CADASTRADOS");
    println!("=================================");

    for cliente in clientes.values() {
        println!("{} - {}", cliente.id, cliente.nome);
    }

    println!("=================================");
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
    println!("=================================");
    println!("       RECOMENDAÇÕES");
    println!("=================================");

    let recomendacoes = grafo.recomendar(
        "Cliente:João",
        produtos_comprados,
    );

    if recomendacoes.is_empty() {
        println!("Nenhuma recomendação encontrada.");
        println!("=================================");
        return;
    }

    println!("Cliente: João");

    for (i, (recomendacao, pontuacao)) in recomendacoes.iter().enumerate() {
        let id = match recomendacao
            .strip_prefix("Produto:")
            .and_then(|valor| valor.parse::<u32>().ok())
        {
            Some(id) => id,
            None => continue,
        };

        if let Some(produto) = produtos.get(&id) {
            println!();
            println!("{}. 🛒 {}", i + 1, produto.nome);
            println!("   Categoria: {}", produto.categoria);
            println!("   Relevância: {}", pontuacao);
        }
    }

    println!();
    println!("=================================");
}

// =========================================
// REGISTRAR COMPRA
// =========================================

fn registrar_compra(
    clientes: &HashMap<u32, Cliente>,
    produtos: &HashMap<u32, Produto>,
    compras: &mut HashMap<u32, HashSet<u32>>,
    grafo: &mut Grafo,
) {
    println!();
    println!("=================================");
    println!("        REGISTRAR COMPRA");
    println!("=================================");

    // =========================================
    // ESCOLHER CLIENTE
    // =========================================

    println!();
    println!("Clientes disponíveis:");

    for cliente in clientes.values() {
        println!("{} - {}", cliente.id, cliente.nome);
    }

    print!("\nDigite o ID do cliente: ");
    io::stdout().flush().unwrap();

    let mut entrada = String::new();

    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler o cliente");

    let cliente_id: u32 = match entrada.trim().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("❌ ID inválido.");
            return;
        }
    };

    // =========================================
    // VERIFICAR CLIENTE
    // =========================================

    let cliente = match clientes.get(&cliente_id) {
        Some(cliente) => cliente,
        None => {
            println!("❌ Cliente não encontrado.");
            return;
        }
    };

    println!();
    println!("Cliente selecionado: {}", cliente.nome);

    // =========================================
    // LISTAR PRODUTOS
    // =========================================

    println!();
    println!("Produtos disponíveis:");

    for produto in produtos.values() {
        println!(
            "{} - {} ({})",
            produto.id,
            produto.nome,
            produto.categoria
        );
    }

    // =========================================
    // REGISTRAR PRODUTOS
    // =========================================

    loop {
        print!("\nDigite o ID do produto: ");
        io::stdout().flush().unwrap();

        entrada.clear();

        io::stdin()
            .read_line(&mut entrada)
            .expect("Erro ao ler o produto");

        let produto_id: u32 = match entrada.trim().parse() {
            Ok(id) => id,
            Err(_) => {
                println!("❌ ID inválido.");
                continue;
            }
        };

        // =========================================
        // VERIFICAR PRODUTO
        // =========================================

        let produto = match produtos.get(&produto_id) {
            Some(produto) => produto,
            None => {
                println!("❌ Produto não encontrado.");
                continue;
            }
        };

        // =========================================
        // REGISTRAR NO HASHMAP
        // =========================================

        let produtos_cliente = compras
            .entry(cliente_id)
            .or_insert_with(HashSet::new);

        // =========================================
        // EVITAR DUPLICIDADE
        // =========================================

        if !produtos_cliente.insert(produto_id) {
            println!();
            println!(
                "⚠️ {} já foi comprado por {}.",
                produto.nome,
                cliente.nome
            );
        } else {
            // =====================================
            // REGISTRAR NO GRAFO
            // =====================================

            grafo.adicionar_aresta(
                format!("Cliente:{}", cliente.nome),
                format!("Produto:{}", produto_id),
            );

            println!();
            println!(
                "✅ {} adicionado à compra de {}.",
                produto.nome,
                cliente.nome
            );
        }

        // =========================================
        // OUTRO PRODUTO?
        // =========================================

        print!("\nDeseja adicionar outro produto? (s/n): ");
        io::stdout().flush().unwrap();

        entrada.clear();

        io::stdin()
            .read_line(&mut entrada)
            .expect("Erro ao ler a opção");

        if entrada.trim().to_lowercase() != "s" {
            break;
        }
    }

    // =========================================
    // RESUMO DA COMPRA
    // =========================================

    println!();
    println!("=================================");
    println!("       COMPRA REGISTRADA");
    println!("=================================");
    println!("Cliente: {}", cliente.nome);
    println!();

    if let Some(produtos_cliente) = compras.get(&cliente_id) {
        println!("Produtos comprados:");

        for produto_id in produtos_cliente {
            if let Some(produto) = produtos.get(produto_id) {
                println!(
                    " - {} ({})",
                    produto.nome,
                    produto.categoria
                );
            }
        }
    }

    println!("=================================");
}

// =========================================
// MOSTRAR COMPRAS
// =========================================

fn mostrar_compras(
    clientes: &HashMap<u32, Cliente>,
    produtos: &HashMap<u32, Produto>,
    compras: &HashMap<u32, HashSet<u32>>,
) {
    println!();
    println!("=================================");
    println!("       COMPRAS REGISTRADAS");
    println!("=================================");

    if compras.is_empty() {
        println!("Nenhuma compra registrada.");
        println!("=================================");
        return;
    }

    for (cliente_id, produtos_comprados) in compras {
        if let Some(cliente) = clientes.get(cliente_id) {
            println!();
            println!("👤 Cliente: {}", cliente.nome);
            println!("   ID: {}", cliente.id);
            println!();
            println!("   Produtos:");

            for produto_id in produtos_comprados {
                if let Some(produto) = produtos.get(produto_id) {
                    println!(
                        "   🛒 {} - {}",
                        produto.id,
                        produto.nome
                    );

                    println!(
                        "      Categoria: {}",
                        produto.categoria
                    );
                }
            }

            println!();
            println!("---------------------------------");
        }
    }

    println!("=================================");
}