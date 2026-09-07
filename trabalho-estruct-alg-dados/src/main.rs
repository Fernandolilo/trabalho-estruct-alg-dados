use std::collections::{HashMap, HashSet, VecDeque};

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

// Representa um vértice do nosso grafo
#[derive(Debug)]
struct Grafo {
    conexoes: HashMap<String, Vec<String>>,
}

impl Grafo {
    // Cria um grafo vazio
    fn novo() -> Self {
        Grafo {
            conexoes: HashMap::new(),
        }
    }

    // Adiciona um vértice
    fn adicionar_vertice(&mut self, vertice: String) {
        self.conexoes.entry(vertice).or_insert(Vec::new());
    }

    // Adiciona uma conexão entre dois vértices
    fn adicionar_aresta(&mut self, origem: String, destino: String) {
        self.conexoes
            .entry(origem.clone())
            .or_insert(Vec::new())
            .push(destino.clone());

        self.conexoes
            .entry(destino)
            .or_insert(Vec::new());
    }

    // Mostra todas as conexões
    fn mostrar(&self) {
        println!("\n=== GRAFO DO CONECTASTORE ===");

        for (vertice, vizinhos) in &self.conexoes {
            println!("{} -> {:?}", vertice, vizinhos);
        }
    }

    // Busca em largura
    fn bfs(&self, inicio: &str) -> Vec<String> {
        let mut fila = VecDeque::new();
        let mut visitados = HashSet::new();
        let mut resultado = Vec::new();

        fila.push_back(inicio.to_string());
        visitados.insert(inicio.to_string());

        while let Some(atual) = fila.pop_front() {
            resultado.push(atual.clone());

            if let Some(vizinhos) = self.conexoes.get(&atual) {
                for vizinho in vizinhos {
                    if !visitados.contains(vizinho) {
                        visitados.insert(vizinho.clone());
                        fila.push_back(vizinho.clone());
                    }
                }
            }
        }

        resultado
    }
}

fn main() {
    println!("=================================");
    println!("       CONECTASTORE");
    println!("=================================");

    // =================================
    // PRODUTOS
    // =================================

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

    produtos.insert(
        4,
        Produto {
            id: 4,
            nome: String::from("Fone de Ouvido"),
            categoria: String::from("Eletrônicos"),
        },
    );

    // =================================
    // CLIENTES
    // =================================

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

    // =================================
    // GRAFO
    // =================================

    let mut grafo = Grafo::novo();

    // Vértices dos clientes
    grafo.adicionar_vertice(String::from("Cliente:João"));
    grafo.adicionar_vertice(String::from("Cliente:Maria"));

    // Vértices dos produtos
    grafo.adicionar_vertice(String::from("Produto:Notebook"));
    grafo.adicionar_vertice(String::from("Produto:Mouse"));
    grafo.adicionar_vertice(String::from("Produto:Teclado"));
    grafo.adicionar_vertice(String::from("Produto:Fone"));

    // =================================
    // RELAÇÕES DE COMPRA
    // =================================

    grafo.adicionar_aresta(
        String::from("Cliente:João"),
        String::from("Produto:Notebook"),
    );

    grafo.adicionar_aresta(
        String::from("Cliente:João"),
        String::from("Produto:Mouse"),
    );

    grafo.adicionar_aresta(
        String::from("Cliente:Maria"),
        String::from("Produto:Notebook"),
    );

    // =================================
    // RELAÇÕES DE SIMILARIDADE
    // =================================

    grafo.adicionar_aresta(
        String::from("Produto:Notebook"),
        String::from("Produto:Teclado"),
    );

    grafo.adicionar_aresta(
        String::from("Produto:Notebook"),
        String::from("Produto:Fone"),
    );

    grafo.adicionar_aresta(
        String::from("Produto:Mouse"),
        String::from("Produto:Teclado"),
    );

    // =================================
    // MOSTRAR DADOS
    // =================================

    println!("\nProdutos cadastrados:");

    for produto in produtos.values() {
        println!(
            "{} - {} ({})",
            produto.id,
            produto.nome,
            produto.categoria
        );
    }

    println!("\nClientes cadastrados:");

    for cliente in clientes.values() {
        println!("{} - {}", cliente.id, cliente.nome);
    }

    // =================================
    // MOSTRAR GRAFO
    // =================================

    grafo.mostrar();

    // =================================
    // BFS
    // =================================

    println!("\n=== PERCURSO BFS ===");

    let percurso = grafo.bfs("Cliente:João");

    for vertice in percurso {
        println!("{}", vertice);
    }
}