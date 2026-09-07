# ConectaStore

## Sistema de Recomendação de Produtos com Estruturas de Dados

O **ConectaStore** é um sistema desenvolvido em **Rust** com o objetivo de demonstrar a aplicação de estruturas de dados na construção de um sistema de recomendação de produtos para um comércio eletrônico.

O projeto utiliza um **grafo** para representar as relações entre clientes e produtos, permitindo percorrer essas conexões e gerar recomendações com base nos relacionamentos existentes.

---

## Objetivo

O principal objetivo do projeto é aplicar, na prática, conceitos de estruturas de dados estudados na disciplina, demonstrando como diferentes estruturas podem trabalhar em conjunto para solucionar um problema real.

O sistema permite:

- Cadastrar e listar produtos;
- Cadastrar e listar clientes;
- Registrar compras;
- Visualizar as compras realizadas;
- Representar clientes e produtos através de um grafo;
- Realizar uma busca em largura (BFS);
- Gerar recomendações de produtos;
- Evitar a recomendação de produtos que o cliente já comprou.

---

## Estruturas de Dados Utilizadas

### HashMap

O `HashMap` é utilizado para armazenar e localizar rapidamente clientes e produtos através de seus IDs.

Exemplo:

```rust
HashMap<u32, Produto>
HashMap<u32, Cliente> 

