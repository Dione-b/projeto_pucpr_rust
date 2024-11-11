use futures::future::join_all; // Import join_all for handling a collection of Futures
use futures::FutureExt;
use reqwasm::http::Request;
use serde::Deserialize;
use std::collections::HashMap;
use wasm_bindgen::prelude::*; // Import FutureExt to use methods like map on Futures

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct ExchangeRatesResponse {
    pub result: String,
    pub documentation: String,
    pub terms_of_use: String,
    pub time_last_update_unix: u64,
    pub time_last_update_utc: String,
    pub time_next_update_unix: u64,
    pub time_next_update_utc: String,
    pub base_code: String,
    pub conversion_rates: HashMap<String, f64>,
}

#[wasm_bindgen]
pub async fn process_csv(csv_content: &str) -> String {
    let mut result = String::new();
    let mut line_count = 0;

    let mut tasks = vec![];

    // Process only the first 10 lines
    for (i, line) in csv_content.lines().take(10).enumerate() {
        let columns: Vec<&str> = line.split(';').collect();

        if columns.len() >= 2 {
            let moeda_origem = columns[0].trim();
            let moeda_destino = columns[1].trim();

            result.push_str(&format!(
                "Linha {}: Moeda Origem = {}, Moeda Destino = {}\n",
                i + 1,
                moeda_origem,
                moeda_destino
            ));

            // Create a task for each line (call fetch_exchange_rate asynchronously)
            let task = fetch_exchange_rate(moeda_origem, moeda_destino).map(|api_response| {
                // We accumulate the response from the API into a string.
                match api_response {
                    Ok((currency, target, rate)) => {
                        format!(
                            "Resposta da API para {} -> {}: {:.2}\n",
                            currency, target, rate
                        )
                    }
                    Err(_) => {
                        format!("Erro ao chamar API para")
                    }
                }
            });

            tasks.push(task);
        } else {
            result.push_str(&format!("Linha {}: Formato inválido\n", i + 1));
        }

        line_count += 1;
    }

    // Await all tasks to finish using join_all
    let responses = join_all(tasks).await;

    // Append the results of the responses to `result`
    for response in responses {
        result.push_str(&response);
    }

    format!(
        "Processamento concluído! Número de linhas: {}\n{}",
        line_count, result
    )
}

async fn fetch_exchange_rate<'a>(
    currency: &'a str,
    target: &'a str,
) -> Result<(&'a str, &'a str, f64), JsValue> {
    let url = format!(
        "https://v6.exchangerate-api.com/v6/41f3c6c0fcad995d00feee53/latest/{}",
        currency
    );

    // Use reqwasm to make the GET request
    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| JsValue::from_str(&format!("Error: {}", e)))?;

    // Get the response text
    let text = response
        .text()
        .await
        .map_err(|e| JsValue::from_str(&format!("Error parsing response: {}", e)))?;

    // Deserialize the JSON response into the ExchangeRatesResponse struct
    let exchange_rate: ExchangeRatesResponse = serde_json::from_str(&text)
        .map_err(|e| JsValue::from_str(&format!("Error deserializing response: {}", e)))?;

    // Filter the conversion rates to get the rate for the specified currency
    match exchange_rate.conversion_rates.get(target) {
        Some(rate) => Ok((currency, target, *rate)), // Return the rate if found
        None => Err(JsValue::from_str("Currency not found in conversion rates")), // Handle case where currency is not found
    }
}
