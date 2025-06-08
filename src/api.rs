//! API client for the quote server.
//! Handles authentication and all CRUD operations for quotes.

use crate::types::{AuthBody, CreateQuoteRequest, QuoteWithTags, Registration, UpdateQuoteRequest};
use reqwasm::http::Request;
use web_sys::window;

// Auth constants
const AUTH_FULL_NAME: &str = "John Smith";
const AUTH_EMAIL: &str = "john@example.com";
const AUTH_PASSWORD: &str = "hepjeq-jatDes-5pykfy";

/// Gets auth token from localStorage or authenticates to get a new one.
pub async fn get_auth_token() -> Result<String, String> {
    let storage = window()
        .and_then(|w| w.local_storage().ok()?)
        .ok_or("localStorage not available")?;

    // Check if token exists in localStorage
    if let Ok(Some(token)) = storage.get_item("auth_token") {
        if !token.is_empty() {
            return Ok(token);
        }
    }

    // If no token, authenticate and store it
    let token = authenticate().await?;
    let _ = storage.set_item("auth_token", &token);

    Ok(token)
}

/// Authenticates with the server and returns an access token.
async fn authenticate() -> Result<String, String> {
    let registration = Registration {
        full_name: AUTH_FULL_NAME.to_string(),
        email: AUTH_EMAIL.to_string(),
        password: AUTH_PASSWORD.to_string(),
    };

    let body = serde_json::to_string(&registration)
        .map_err(|e| format!("Failed to serialize request: {:?}", e))?;

    let resp = Request::post("http://localhost:3000/auth")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {:?}", e))?;

    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }

    let auth_body: AuthBody = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON: {:?}", e))?;

    Ok(auth_body.access_token)
}

/// Fetches all quotes from the server.
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

/// Fetches a specific quote by ID.
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

/// Fetches a random quote from the server.
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

/// Creates a new quote (requires authentication).
pub async fn create_quote(request: CreateQuoteRequest) -> Result<QuoteWithTags, String> {
    let token = get_auth_token().await?;
    let body = serde_json::to_string(&request)
        .map_err(|e| format!("Failed to serialize request: {:?}", e))?;

    let resp = Request::post("http://localhost:3000/api/v1/quotes")
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
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

/// Updates an existing quote (requires authentication).
pub async fn update_quote(id: i64, request: UpdateQuoteRequest) -> Result<QuoteWithTags, String> {
    let token = get_auth_token().await?;
    let body = serde_json::to_string(&request)
        .map_err(|e| format!("Failed to serialize request: {:?}", e))?;

    let url = format!("http://localhost:3000/api/v1/quotes/{}", id);
    let resp = Request::put(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
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

/// Deletes a quote by ID (requires authentication).
pub async fn delete_quote(id: i64) -> Result<(), String> {
    let token = get_auth_token().await?;
    let url = format!("http://localhost:3000/api/v1/quotes/{}", id);
    let resp = Request::delete(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Request failed: {:?}", e))?;

    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }

    Ok(())
}
