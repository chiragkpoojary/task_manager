use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct SignInData {
    name: String,
    password: String,
}
