//! Type definitions for API request/response structures.

use serde::{Deserialize, Serialize};

/// Quote data structure with associated tags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteWithTags {
    pub id: i64,
    pub quote: String,
    pub source: String,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Vec<String>,
}

/// Request payload for creating a new quote.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQuoteRequest {
    pub quote: String,
    pub source: String,
    pub tags: Option<Vec<String>>,
}

/// Request payload for updating an existing quote.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateQuoteRequest {
    pub quote: String,
    pub source: String,
    pub tags: Option<Vec<String>>,
}

/// User registration data for authentication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registration {
    pub full_name: String,
    pub email: String,
    pub password: String,
}

/// Authentication response containing access token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}
