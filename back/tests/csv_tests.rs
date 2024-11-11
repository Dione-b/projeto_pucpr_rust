use wasm_bindgen_test::wasm_bindgen_test;
use wasm_bindgen_test::wasm_bindgen_test_configure;

wasm_bindgen_test_configure!(run_in_browser);

use back::*; // Altere `back` pelo nome do seu crate

#[wasm_bindgen_test]
async fn test_process_csv_with_valid_data() {
    let csv_content = "USD;EUR\nBRL;JPY";
    let result = process_csv(csv_content).await;
    assert!(result.contains("Linha 1: Moeda Origem = USD, Moeda Destino = EUR"));
    assert!(result.contains("Linha 2: Moeda Origem = BRL, Moeda Destino = JPY"));
}

#[wasm_bindgen_test]
async fn test_process_csv_with_invalid_line_format() {
    let csv_content = "USD;EUR\nINVALID_LINE";
    let result = process_csv(csv_content).await;
    assert!(result.contains("Linha 2: Formato inválido"));
}

#[wasm_bindgen_test]
async fn test_process_csv_with_exchange_rate_response() {
    let csv_content = "USD;EUR\nBRL;USD";
    let result = process_csv(csv_content).await;
    assert!(result.contains("Resposta da API para USD -> EUR"));
    assert!(result.contains("Resposta da API para BRL -> USD"));
}

#[wasm_bindgen_test]
async fn test_process_csv_with_invalid_currency() {
    let csv_content = "XYZ;EUR";
    let result = process_csv(csv_content).await;
    assert!(result.contains("Erro ao chamar API para"));
}

#[wasm_bindgen_test]
async fn test_process_csv_large_file() {
    let csv_content = "USD;EUR\n".repeat(15); // Cria um CSV com 15 linhas
    let result = process_csv(&csv_content).await;
    assert!(result.contains("Processamento concluído! Número de linhas: 10"));
}

#[wasm_bindgen_test]
async fn test_process_csv_empty_file() {
    let csv_content = "";
    let result = process_csv(csv_content).await;
    assert!(result.contains("Processamento concluído! Número de linhas: 0"));
}

#[wasm_bindgen_test]
async fn test_process_csv_with_mixed_valid_and_invalid_lines() {
    let csv_content = "USD;EUR\nINVALID_LINE\nBRL;JPY\nBRL";
    let result = process_csv(csv_content).await;
    assert!(result.contains("Linha 2: Formato inválido"));
    assert!(result.contains("Linha 4: Formato inválido"));
    assert!(result.contains("Linha 1: Moeda Origem = USD, Moeda Destino = EUR"));
    assert!(result.contains("Linha 3: Moeda Origem = BRL, Moeda Destino = JPY"));
}
