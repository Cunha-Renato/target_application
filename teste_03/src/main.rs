#[derive(Debug, Default)]
struct EstatisticaMensal {
    menor_faturamento_dia: u64,
    maior_faturamento_dia: u64,
    quantidade_dias_acima_media: u32,
    media_mensal: f64,
}

fn faturamento_mensal(value: serde_json::Value) -> EstatisticaMensal {
    let mut result = EstatisticaMensal::default();
    
    if let Some(faturamento_diario) = value["faturamento_diario"].as_array() {
        // Calculo da média
        result.media_mensal = faturamento_diario
            .iter()
            .filter(|dia| dia["valor"].as_f64().unwrap() != 0.0)
            .map(|dia| dia["valor"].as_f64().unwrap())
            .sum::<f64>() / faturamento_diario
            .iter()
            .filter(|dia| dia["valor"].as_f64().unwrap() != 0.0)
            .count() as f64;
        
        let mut menor_faturamento = f64::MAX;
        let mut maior_faturamento = 0.0;

        for dia in faturamento_diario {
            let dia_num = dia["dia"].as_u64().unwrap();
            let faturamento = dia["valor"].as_f64().unwrap();
            if faturamento != 0.0 {
                if menor_faturamento > faturamento {
                    menor_faturamento = faturamento;
                    result.menor_faturamento_dia = dia_num;
                }

                if maior_faturamento < faturamento {
                    maior_faturamento = faturamento;
                    result.maior_faturamento_dia = dia_num;
                }
                
                if faturamento > result.media_mensal {
                    result.quantidade_dias_acima_media += 1;
                }
            }
        }
    }
        
    result
}

fn main() {
    let abril = std::fs::read_to_string("abril.json").unwrap();
    let value: serde_json::Value = serde_json::from_str(&abril).unwrap();
    
    println!("{:#?}", faturamento_mensal(value));
}
