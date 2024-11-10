use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// Define a estrutura para cada moeda no JSON
#[derive(Deserialize)]
struct Moeda {
    codigo: String,
    cotacao: f64,
    #[warn(dead_code)]
    variacao: f64,
}

// Define a estrutura principal que contém a lista de moedas
#[derive(serde::Deserialize)]
struct ExchangeRates {
    moedas: Vec<Moeda>,
}

// Função para converter o valor de entrada para a moeda desejada
#[wasm_bindgen]
pub fn convert_currency(
    json_data: &str,
    amount_str: &str,
    target_currency: &str,
) -> Result<String, JsValue> {
    // Tenta parsear o JSON com as cotações
    let exchange_data: Result<ExchangeRates, _> = serde_json::from_str(json_data);

    match exchange_data {
        Ok(data) => {
            // Converte a string de quantidade para um número
            let amount: f64 = amount_str
                .parse()
                .map_err(|_| JsValue::from_str("Valor de entrada inválido"))?;

            // Busca a taxa de câmbio para a moeda desejada
            let moeda = data
                .moedas
                .iter()
                .find(|&m| m.codigo == target_currency)
                .ok_or_else(|| JsValue::from_str("Moeda não encontrada"))?;

            // Calcula o valor convertido
            let converted_value = amount / moeda.cotacao;

            // Retorna o valor convertido como string
            Ok(format!("{:.2}", converted_value))
        }
        Err(e) => {
            // Se o parse falhar, retorna o erro real do parse
            Err(JsValue::from_str(&e.to_string()))
        }
    }
}
