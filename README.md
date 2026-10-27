# Amazon Engineering Challenge (EDBII)

## Implementação e Benchmark de Árvores Balanceadas (AVL vs LLRB) em Rust

Este repositório contém uma implementação robusta, segura para a memória e modular de duas das principais estruturas de dados de Árvores Binárias de Busca Balanceadas: a **Árvore AVL** e a **Árvore Rubro-Negra Inclinada à Esquerda (LLRB)** (baseada no algoritmo de Robert Sedgewick).

O projeto foi desenvolvido com foco estrito em validação matemática de propriedades e análise de performance (benchmarking) em cenários de pior e melhor caso.

## 🚀 Funcionalidades

* **Árvore AVL (`AVLTree`):** Balanceamento rigoroso baseado em altura (Fator de Balanceamento).
* **Árvore Rubro-Negra LLRB (`RBTree`):** Balanceamento focado em repintura de nós e inclinação à esquerda, otimizando inserções contínuas.
* **Operações Core:** Inserção, Busca (`contains`), Remoção (tratando folhas, nós com um filho e nós com dois filhos através da extração do mínimo).
* **Validação Estrutural Automatizada:** Métodos internos que provam matematicamente que as propriedades das árvores não foram corrompidas após as mutações (validação de Fator de Balanceamento para AVL; validação de raiz preta, ausência de duplo vermelho e Altura Negra para LLRB).
* **Métricas Coletadas:** Tempo de execução (microssegundos), quantidade exata de rotações de ponteiros e altura final da estrutura.
* **Visualização no Terminal:** Método `show_tree()` para imprimir a topologia da árvore deitada, facilitando o debug visual.
* **Console iterativo:** permite escolher interagir com árvore RB(LLRB) e AVL pelo terminal. O usuário pode usar todas as operações já ditas anteriormente. Além delas, pode escolher ver a árvore em ordem(`in_order()`) e usar o método `show_tree()` para observar a estrutura das árvores.

## 📂 Estrutura do Projeto

O código está organizado de forma modular, tirando proveito do sistema de arquivos do Rust:

```text
meu_projeto/
├── Cargo.toml
└── src/
    ├── main.rs   # Ponto de entrada: orquestra a suíte de testes, benchmark e a interatividade.
    ├── avl.rs    # Módulo contendo as structs e a lógica da Árvore AVL
    └── rb.rs     # Módulo contendo as structs e a lógica da Árvore Rubro-Negra (LLRB)

```

## 🛠️ Dependências

O projeto utiliza bibliotecas externas para geração de dados aleatórios e exportação de resultados. Certifique-se de que o seu `Cargo.toml` possui as seguintes dependências:

```toml
[dependencies]
csv = "1.3"
rand = "0.8"

```

## ⚙️  Como Executar

Para ter as métricas do benchmark se aproximando de valores mais reais, é **crucial** que seja utilizado todas as _flags_ de otimização do LLVM.

```bash
cargo run --release

```

## 🖥️  Durante a execução
### Ao rodar o código, o o menu permite ao usuário escolher entre 4 opções:

1. Sair do programa
2. Executar testes automáticos e gerar .csv com dados
3. Interagir com árvore AVL
4. Interagir com árvore RB

### Ao entrar nos menus **interativos**, o usuário tem as seguintes opções:

1. Voltar ao menu inicial
2. Adicionar elemento
3. Remover elemento
4. Buscar elemento
5. Mostrar elementos em ordem(in order)
6. Mostrar estrutura da árvore

A interatividade tem como função demonstrar de modo simples o funcionamento das estruturas.

## 📊 Sobre os Benchmarks

O benchmark compara a AVL e a LLRB sob os mesmos conjuntos de dados nos tamanhos de **10.000, 50.000 e 100.000** elementos, nos seguintes cenários:

* **Ordem Crescente:** Pior caso absoluto de inserção (força rebalanceamento contínuo para a esquerda).
* **Ordem Decrescente:** Pior caso reverso (força rebalanceamento para a direita).
* **Não Ordenada (Aleatória):** Caso médio prático (usa dados previamente embaralhados com `rand`).

**Dados Exportados no CSV:**

1. `Estrutura` (AVL ou RB_Tree)
2. `Tamanho` do Dataset
3. `Ordem` de Inserção
4. `Operacao` (Inserção, Busca, Remoção)
5. `Tempo_us` (Duração em microssegundos)
6. `Rotacoes` (Quantidade de rotações estruturais executadas)
7. `Altura_Arvore` (Profundidade final atingida na memória)

## 🧠 Detalhes Técnicos da Implementação

* **Memory Safety:** O projeto evita completamente o uso de clones desnecessários e `unsafe`. A navegação pelas árvores e as transferências de propriedade durante as rotações utilizam `Option::take()` e `unwrap()` de forma segura, honrando o Borrow Checker do Rust.
* **Remoção na LLRB:** A remoção na Rubro-Negra segue o rigoroso algoritmo de Sedgewick, forçando a cor vermelha para baixo (`move_red_left` / `move_red_right`) para garantir que os nós excluídos nunca prejudiquem a contagem da Altura Negra.
* **Prevenção de Corrupção:** A função `remove` da RBTree possui uma checagem preliminar assintótica de existência (`contains`) para evitar que a tentativa de remover elementos fantasmas corrompa a pintura e o balanço da árvore.
