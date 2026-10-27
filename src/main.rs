mod avl;
mod rb;

use rand::prelude::*;
use rand::seq::SliceRandom;

use std::error::Error;
use std::fs::File;
use std::io::{Write, stdin, stdout};
use std::time::Instant;

use avl::AVLTree;
use rb::RBTree;

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        println!("============================");
        println!(" AMAZON LOGISTICS CHALLENGE");
        println!("============================\n");

        println!("0. Sair do programa");
        println!("1. Executar testes automáticos e gerar .csv com dados");
        println!("2. Interagir com árvore AVL");
        println!("3. Interagir com árvore RB");
        print!("Digite a opção: ");
        stdout().flush()?;

        let input_opt = get_input_stdin();
        print!("\n");

        if input_opt == 1 {
            automated_tests()?;
        } else if input_opt == 2 {
            avl_tree_interactive()?;
        } else if input_opt == 3 {
            rb_tree_interactive()?;
        } else {
            return Ok(());
        }
    }
}

fn avl_tree_interactive() -> Result<(), Box<dyn Error>> {
    let mut avl: AVLTree<i32> = AVLTree::new();
    loop {
        println!("Interagindo com árvore AVL");
        println!("0. Voltar ao menu inicial");
        println!("1. Adicionar elemento");
        println!("2. Remover elemento");
        println!("3. Buscar elemento");
        println!("4. Mostrar elementos em ordem(in order)");
        println!("5. Mostrar estrutura da árvore");
        print!("Digite a opção: ");
        stdout().flush()?;

        let input_opt = get_input_stdin();

        print!("\n");
        match input_opt {
            1 => {
                println!("Inserindo elementos");
                print!("Digite o número: ");
                stdout().flush()?;

                let input_num = get_input_stdin();
                avl.insert(input_num);
            }
            2 => {
                println!("Removendo elementos");
                print!("Digite o número: ");
                stdout().flush()?;

                let input_num = get_input_stdin();
                avl.remove(input_num);
            }
            3 => {
                println!("Buscando elementos");
                print!("Digite o número: ");
                stdout().flush()?;

                let input_num = get_input_stdin();
                if avl.contains(&input_num) {
                    println!("A árvore contém o elemento especificado")
                } else {
                    println!("A árvore não contém o elemento especificado")
                }
                println!();
            }
            4 => {
                let in_ord = avl.in_order();
                println!("{:?}\n", in_ord);
            }
            5 => {
                println!("Árvore AVL");
                avl.show_tree();
                println!();
            }
            _ => {
                return Ok(());
            }
        }
    }
}

fn rb_tree_interactive() -> Result<(), Box<dyn Error>> {
    let mut rb: RBTree<i32> = RBTree::new();
    loop {
        println!("Interagindo com árvore RB");
        println!("0. Voltar ao menu inicial");
        println!("1. Adicionar elemento");
        println!("2. Remover elemento");
        println!("3. Buscar elemento");
        println!("4. Mostrar elementos em ordem(in order)");
        println!("5. Mostrar estrutura da árvore");
        print!("Digite a opção: ");
        stdout().flush()?;

        let input_opt = get_input_stdin();
        print!("\n");
        match input_opt {
            1 => {
                print!("Digite o número: ");
                stdout().flush()?;

                let input_num = get_input_stdin();
                rb.insert(input_num);
            }
            2 => {
                print!("Digite o número: ");
                stdout().flush()?;

                let input_num = get_input_stdin();
                rb.remove(input_num);
            }
            3 => {
                print!("Digite o número: ");
                stdout().flush()?;

                let input_num = get_input_stdin();
                if rb.contains(&input_num) {
                    println!("A árvore contém o elemento especificado")
                } else {
                    println!("A árvore não contém o elemento especificado")
                }
                println!();
            }
            4 => {
                let in_ord = rb.in_order();
                println!("{:?}\n", in_ord);
            }
            5 => {
                println!("Árvore Rubro-Negra:");
                rb.show_tree();
                println!();
            }
            _ => {
                return Ok(());
            }
        }
    }
}

fn automated_tests() -> Result<(), Box<dyn Error>> {
    println!("Verificando requisitos das árvores");
    testes_funcionais();

    println!("Fazendo benchmarks e guardando em um .csv");
    executar_benchmark()?;

    Ok(())
}

fn testes_funcionais() {
    let mut avl = AVLTree::new();
    let mut rb = RBTree::new();

    println!("Testando árvores vazias e com 1 elemento");

    assert_eq!(avl.contains(&10), false);
    assert_eq!(rb.contains(&10), false);

    avl.remove(10);
    rb.remove(10);

    avl.insert(10);
    rb.insert(10);

    assert_eq!(avl.contains(&10), true);
    assert_eq!(rb.contains(&10), true);

    assert!(
        avl.validate_properties(),
        "AVL quebrou a propriedade na raiz única"
    );
    assert!(
        rb.validate_properties(),
        "RB quebrou a propriedade na raiz única"
    );

    println!("Testando inserções e propriedades estruturais da árvore");
    let dados = [5, 15, 3, 7, 12, 18, 1, 4];
    for &d in &dados {
        avl.insert(d);
        rb.insert(d);

        assert!(
            avl.validate_properties(),
            "AVL quebrou a propriedade na inserção de {}",
            d
        );
        assert!(
            rb.validate_properties(),
            "RB quebrou a propriedade na inserção de {}",
            d
        );
    }

    println!("Testando função in_order");
    let avl_ord = avl.in_order();
    let rb_ord = rb.in_order();

    for i in 1..avl_ord.len() {
        assert!(
            avl_ord[i - 1] < avl_ord[i],
            "in_order da árvore AVL está com erro"
        );
    }

    for i in 1..rb_ord.len() {
        assert!(
            rb_ord[i - 1] < rb_ord[i],
            "in_order da árvore Rubro-Negra está com erro"
        );
    }

    println!("Mostrando impressão das duas árvores");
    println!("Árvore AVL:");
    avl.show_tree();

    println!("\nÁrvore Rubro-Negra:");
    rb.show_tree();

    println!();

    println!("Testando buscas (com elementos existentes e inexistentes)");
    assert_eq!(avl.contains(&15), true); // Existente
    assert_eq!(rb.contains(&15), true); // Existente

    assert_eq!(avl.contains(&99), false); // Inexistente
    assert_eq!(rb.contains(&99), false); // Inexistente

    println!("Testando remoção de elemento inexistente");
    avl.remove(99);
    rb.remove(99);

    assert!(
        avl.validate_properties(),
        "AVL corrompida após tentar remover inexistente"
    );

    assert!(
        rb.validate_properties(),
        "RB corrompida após tentar remover inexistente"
    );

    println!("-> Testando remoções (Folha, 1 Filho, 2 Filhos)...");
    let remover_dados = [1, 18, 5, 10];

    for &r in &remover_dados {
        avl.remove(r);
        rb.remove(r);

        assert_eq!(avl.contains(&r), false, "AVL não deletou o elemento {}", r);
        assert_eq!(rb.contains(&r), false, "RB não deletou o elemento {}", r);

        assert!(
            avl.validate_properties(),
            "AVL corrompida após remover {}",
            r
        );
        assert!(rb.validate_properties(), "RB corrompida após remover {}", r);
    }

    println!(
        "Todos os testes funcionais e de integridade passaram com sucesso em ambas as árvores!"
    );
}

fn executar_benchmark() -> Result<(), Box<dyn Error>> {
    let file = File::create("benchmark_completo.csv")?;
    let mut wtr = csv::Writer::from_writer(file);

    wtr.write_record(&[
        "Estrutura",
        "Tamanho",
        "Ordem",
        "Operacao",
        "Tempo_us",
        "Rotacoes",
        "Altura_Arvore",
    ])?;

    let tamanhos = [10_000, 50_000, 100_000];
    let mut rng = ThreadRng::default();

    for &tamanho in &tamanhos {
        let dados_crescente: Vec<i32> = (0..tamanho).collect();
        let dados_decrescente: Vec<i32> = (0..tamanho).rev().collect();

        let mut dados_aleatorios = dados_crescente.clone();
        dados_aleatorios.shuffle(&mut rng);

        let cenarios = [
            ("Crescente", &dados_crescente),
            ("Decrescente", &dados_decrescente),
            ("Nao_Ordenada", &dados_aleatorios),
        ];

        for (nome_ordem, dataset) in cenarios {
            println!("Testando {} com {} dados...", nome_ordem, tamanho);

            // Testes AVL
            let mut avl = AVLTree::new();

            // Inserção
            avl.rotacoes = 0;
            let inicio = Instant::now();
            for &v in dataset {
                avl.insert(v);
            }
            let tempo_ins = inicio.elapsed().as_micros();
            wtr.write_record(&[
                "AVL",
                &tamanho.to_string(),
                nome_ordem,
                "Insercao",
                &tempo_ins.to_string(),
                &avl.rotacoes.to_string(),
                &avl.height().to_string(),
            ])?;

            // Busca
            let inicio = Instant::now();
            for &v in dataset {
                avl.contains(&v);
            }
            let tempo_busca = inicio.elapsed().as_micros();
            wtr.write_record(&[
                "AVL",
                &tamanho.to_string(),
                nome_ordem,
                "Busca",
                &tempo_busca.to_string(),
                "0",
                &avl.height().to_string(),
            ])?;

            // Remoção
            avl.rotacoes = 0;
            let inicio = Instant::now();
            for &v in dataset {
                avl.remove(v);
            }
            let tempo_rem = inicio.elapsed().as_micros();
            wtr.write_record(&[
                "AVL",
                &tamanho.to_string(),
                nome_ordem,
                "Remocao",
                &tempo_rem.to_string(),
                &avl.rotacoes.to_string(),
                "0",
            ])?;

            // Testes Rubro-Negra (LLRB)
            let mut rb = RBTree::new();

            // Inserção
            rb.rotations = 0;
            let inicio = Instant::now();
            for &v in dataset {
                rb.insert(v);
            }
            let tempo_ins = inicio.elapsed().as_micros();
            wtr.write_record(&[
                "RB_Tree",
                &tamanho.to_string(),
                nome_ordem,
                "Insercao",
                &tempo_ins.to_string(),
                &rb.rotations.to_string(),
                &rb.height().to_string(),
            ])?;

            // Busca
            let inicio = Instant::now();
            for &v in dataset {
                rb.contains(&v);
            }
            let tempo_busca = inicio.elapsed().as_micros();
            wtr.write_record(&[
                "RB_Tree",
                &tamanho.to_string(),
                nome_ordem,
                "Busca",
                &tempo_busca.to_string(),
                "0",
                &rb.height().to_string(),
            ])?;

            // Remoção
            rb.rotations = 0;
            let inicio = Instant::now();
            for &v in dataset {
                rb.remove(v);
            }
            let tempo_rem = inicio.elapsed().as_micros();
            wtr.write_record(&[
                "RB_Tree",
                &tamanho.to_string(),
                nome_ordem,
                "Remocao",
                &tempo_rem.to_string(),
                &rb.rotations.to_string(),
                "0",
            ])?;
        }
    }

    wtr.flush()?;
    println!("Resultados gerados em 'benchmark_completo.csv'");
    Ok(())
}

fn get_input_stdin() -> i32 {
    let input_opt: i32;
    loop {
        let mut input_line = String::new();

        stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        input_opt = if let Ok(x) = input_line.trim().parse() {
            x
        } else {
            println!("Erro! Por favor digite um número inteiro");
            continue;
        };
        break;
    }
    input_opt
}
