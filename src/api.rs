use crate::types::QuoteWithTags;
use serde_json;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

pub async fn fetch_quotes() -> Result<Vec<QuoteWithTags>, String> {
    let opts = RequestInit::new();
    opts.set_method("GET");

    let request = Request::new_with_str_and_init("http://localhost:3000/api/v1/quotes", &opts)
        .map_err(|_| "Failed to create request")?;

    let window = web_sys::window().ok_or("No global window exists")?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "Request failed")?;

    let resp: Response = resp_value
        .dyn_into()
        .map_err(|_| "Failed to cast response")?;

    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }

    let json = JsFuture::from(resp.json().map_err(|_| "Failed to get JSON")?)
        .await
        .map_err(|_| "Failed to parse JSON")?;

    let json_string = js_sys::JSON::stringify(&json)
        .map_err(|_| "Failed to stringify JSON")?;

    let json_str = json_string.as_string().ok_or("Failed to convert to string")?;

    serde_json::from_str::<Vec<QuoteWithTags>>(&json_str)
        .map_err(|e| format!("Failed to deserialize: {}", e))
}