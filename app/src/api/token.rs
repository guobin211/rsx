use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    /// token expiration time
    static ref MAX_EXP: usize = 7 * 24 * 60 * 60;
    /// token secret key
    static ref SECRET_KEY: String = "generate_token".to_string();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInput {
    pub id: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload {
    /// user id
    pub id: String,
    /// username
    pub username: String,
    /// expiration time
    exp: usize,
}

/// Generates a JSON Web Token (JWT) for a given user.
///
/// # Arguments
///
/// * `user` - A reference to a `TokenInput` struct containing the user's ID and username.
///
/// # Returns
///
/// * `Option<String>` - Returns `Some(String)` containing the generated token if successful, or `None` if the user data is invalid or token generation fails.
///
/// # Example
///
/// ```
/// let user = TokenInput {
///     id: "1".to_string(),
///     username: "admin666".to_string(),
/// };
/// if let Some(token) = generate_token(&user) {
///     println!("token: {}", token);
/// }
/// ```
pub fn generate_token(user: &TokenInput) -> Option<String> {
    // Check if the user data is valid
    if user.username.is_empty() || user.id.is_empty() {
        return None;
    }

    // Create an encoding key from the secret key
    let sec = EncodingKey::from_secret(SECRET_KEY.as_ref());

    // Get the current timestamp
    let now = chrono::Utc::now().timestamp() as usize;

    // Create a token payload with the user data and expiration time
    let tp = TokenPayload {
        id: user.id.clone(),
        username: user.username.clone(),
        exp: *MAX_EXP + now,
    };

    // Encode the token payload into a JWT
    if let Ok(token) = encode(&Header::default(), &tp, &sec) {
        return Some(token);
    }

    // Return None if token generation fails
    None
}

// 校验token
pub fn check_token_exp(token: &str) -> Option<TokenPayload> {
    let sec = DecodingKey::from_secret(SECRET_KEY.as_ref());
    let res = decode::<TokenPayload>(token, &sec, &Validation::new(Algorithm::HS256));
    match res {
        Ok(tp) => {
            let now = chrono::Utc::now().timestamp() as usize;
            if tp.claims.exp > now {
                return Some(tp.claims);
            }
            None
        }
        Err(e) => {
            log::error!("check token error: {}", e);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_token() {
        let user = TokenInput {
            id: "1".to_string(),
            username: "admin666".to_string(),
        };
        if let Some(token) = generate_token(&user) {
            println!("token: {}", token);
            assert!(!token.is_empty());
        }
    }

    #[test]
    fn test_check_token() {
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjEiLCJ1c2VybmFtZSI6ImFkbWluNjY2IiwiZXhwIjoxNzAzNjYyOTIzfQ.jeLig-hUaQvnWjOx7TF3wjTX9UiCjzUfPulnVN8phKU";
        if let Some(tp) = check_token_exp(token) {
            println!("token payload: {:?}", tp);
            assert_eq!(tp.id, "1");
            assert_eq!(tp.username, "admin666");
        } else {
            println!("token is expired");
        }
    }
}
