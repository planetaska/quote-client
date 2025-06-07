use crate::types::{QuoteWithTags, CreateQuoteRequest, UpdateQuoteRequest};
use reqwasm::http::Request;

pub async fn fetch_quotes() -> Result<Vec<QuoteWithTags>, String> {
    let resp = Request::get("http://localhost:3000/api/v1/quotes")
        .send()
        .await
        .map_err(|e| format!("Request failed: {:?}", e))?;

    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }

    resp.json::<Vec<QuoteWithTags>>()
        .await
        .map_err(|e| format!("Failed to parse JSON: {:?}", e))
}

pub async fn fetch_quote_by_id(id: u32) -> Result<QuoteWithTags, String> {
    let url = format!("http://localhost:3000/api/v1/quotes/{}", id);
    let resp = Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Request failed: {:?}", e))?;

    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }

    resp.json::<QuoteWithTags>()
        .await
        .map_err(|e| format!("Failed to parse JSON: {:?}", e))
}

pub async fn fetch_random_quote() -> Result<QuoteWithTags, String> {
    let resp = Request::get("http://localhost:3000/api/v1/quotes/random")
        .send()
        .await
        .map_err(|e| format!("Request failed: {:?}", e))?;

    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }

    resp.json::<QuoteWithTags>()
        .await
        .map_err(|e| format!("Failed to parse JSON: {:?}", e))
}

pub async fn create_quote(request: CreateQuoteRequest) -> Result<QuoteWithTags, String> {
    let body = serde_json::to_string(&request)
        .map_err(|e| format!("Failed to serialize request: {:?}", e))?;
    
    let resp = Request::post("http://localhost:3000/api/v1/quotes")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {:?}", e))?;

    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }

    resp.json::<QuoteWithTags>()
        .await
        .map_err(|e| format!("Failed to parse JSON: {:?}", e))
}

pub async fn update_quote(id: i64, request: UpdateQuoteRequest) -> Result<QuoteWithTags, String> {
    let body = serde_json::to_string(&request)
        .map_err(|e| format!("Failed to serialize request: {:?}", e))?;
    
    let url = format!("http://localhost:3000/api/v1/quotes/{}", id);
    let resp = Request::put(&url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {:?}", e))?;

    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }

    resp.json::<QuoteWithTags>()
        .await
        .map_err(|e| format!("Failed to parse JSON: {:?}", e))
}