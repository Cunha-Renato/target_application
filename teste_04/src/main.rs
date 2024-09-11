#[derive(Debug)]
struct Estado {
    nome: String,
    faturamento: f64,    
}

fn participacao_mensal(estados: &[Estado]) {
    let faturamento_total = estados
        .iter()
        .map(|e| e.faturamento)
        .sum::<f64>();
    
    estados
        .iter()
        .for_each(|e| {
            let participacao = (e.faturamento * 100.0) / faturamento_total;
            println!("Estado: {}, participacao: {:.2}%", e.nome, participacao);
        });
}

fn main() {
    let estados = vec![
        Estado {
            nome: "SP".to_string(),
            faturamento: 67_678.43,
        },
        Estado {
            nome: "RJ".to_string(),
            faturamento: 36_678.66,
        },
        Estado {
            nome: "MG".to_string(),
            faturamento: 29_229.88,
        },
        Estado {
            nome: "ES".to_string(),
            faturamento: 27_165.48,
        },
        Estado {
            nome: "Outros".to_string(),
            faturamento: 19_849.53,
        },
    ];
    
    participacao_mensal(&estados);
}
