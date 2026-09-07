use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Grafo {
    conexoes: HashMap<String, Vec<String>>,
}

impl Grafo {
    pub fn novo() -> Self {
        Grafo {
            conexoes: HashMap::new(),
        }
    }

    pub fn adicionar_vertice(&mut self, vertice: String) {
        self.conexoes.entry(vertice).or_insert(Vec::new());
    }

    pub fn adicionar_aresta(&mut self, origem: String, destino: String) {
        self.conexoes
            .entry(origem.clone())
            .or_insert(Vec::new())
            .push(destino.clone());

        self.conexoes
            .entry(destino)
            .or_insert(Vec::new());
    }

    pub fn mostrar(&self) {
        println!("\n=== GRAFO DO CONECTASTORE ===");

        for (vertice, vizinhos) in &self.conexoes {
            println!("{} -> {:?}", vertice, vizinhos);
        }
    }

    pub fn bfs(&self, inicio: &str) -> Vec<String> {
        let mut fila = VecDeque::new();
        let mut visitados = HashSet::new();
        let mut resultado = Vec::new();

        if !self.conexoes.contains_key(inicio) {
            return resultado;
        }

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

    pub fn recomendar(
        &self,
        cliente: &str,
        produtos_comprados: &HashSet<String>,
    ) -> Vec<(String, usize)> {
        let mut recomendacoes: HashMap<String, usize> = HashMap::new();

        let percurso = self.bfs(cliente);

        for vertice in percurso {
            if !vertice.starts_with("Produto:") {
                continue;
            }

            if produtos_comprados.contains(&vertice) {
                continue;
            }

            let pontuacao = self
                .conexoes
                .values()
                .filter(|vizinhos| vizinhos.contains(&vertice))
                .count();

            recomendacoes.insert(vertice, pontuacao);
        }

        let mut resultado: Vec<(String, usize)> =
            recomendacoes.into_iter().collect();

        resultado.sort_by(|a, b| b.1.cmp(&a.1));

        resultado
    }
}