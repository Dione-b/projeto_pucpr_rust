use reqwasm::http::Request;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn process_csv(csv_content: &str) -> String {
    let mut result = String::new();
    let mut line_count = 0;

    for line in csv_content.lines() {
        let columns: Vec<&str> = line.split(';').collect();

        if columns.len() >= 2 {
            let moeda_origem = columns[0].trim();
            let moeda_destino = columns[1].trim();

            result.push_str(&format!(
                "Linha {}: Moeda Origem = {}, Moeda Destino = {}\n",
                line_count + 1,
                moeda_origem,
                moeda_destino
            ));

            // Faz a chamada para a API
            match fetch_exchange_rate(moeda_origem).await {
                Ok(api_response) => {
                    result.push_str(&format!("Resposta da API: {}\n", api_response));
                }
                Err(e) => {
                    result.push_str(&format!("Erro ao chamar API\n"));
                }
            }
        } else {
            result.push_str(&format!("Linha {}: Formato inválido\n", line_count + 1));
        }

        line_count += 1;
    }

    format!(
        "Processamento concluído! Número de linhas: {}\n{}",
        line_count, result
    )
}

async fn fetch_exchange_rate(currency: &str) -> Result<String, JsValue> {
    let url = format!(
        "https://v6.exchangerate-api.com/v6/41f3c6c0fcad995d00feee53/latest/{}",
        currency
    );
    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| JsValue::from_str(&format!("Error: {}", e)))?;

    let text = response
        .text()
        .await
        .map_err(|e| JsValue::from_str(&format!("Error parsing response: {}", e)))?;

    Ok(text)
}
