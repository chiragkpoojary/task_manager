use crate::db::db_struct::AuthResponse;
use crate::db::db_struct::AuthRequest;
use reqwest::Client;
use std::error::Error;

pub async fn sign_in(
    client: &Client,
    api_url: &str,
    api_key: &str,
    email: &str,
    password: &str,
) -> Result<AuthResponse, Box<dyn Error>> {
    let request_body = AuthRequest {
        email: email.to_string(),
        password: password.to_string(),
    };

    let response = client
        .post(&format!("{}/auth/v1/token?grant_type=password", api_url))
        .header("apikey", api_key)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?
        .json::<AuthResponse>()
        .await?;

    Ok(response)
}


