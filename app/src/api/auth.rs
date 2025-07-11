use std::collections::HashMap;
use std::sync::Mutex;

use actix_web::{HttpRequest, HttpResponse, cookie::Cookie, http::StatusCode, web};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::api::token::{TokenInput, check_token_exp, generate_token};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct User {
    id: String,
    username: String,
    password: String,
    email: String,
}

lazy_static! {
    static ref MAX_EXP: usize = 7 * 24 * 60 * 60;
    static ref TOKEN_MAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref USER_MAP: Mutex<HashMap<String, User>> = {
        let mut map = HashMap::new();
        map.insert(
            "admin666".to_string(),
            User {
                id: "1".to_string(),
                username: "admin666".to_string(),
                password: "admin666".to_string(),
                email: "".to_string(),
            },
        );
        map.insert(
            "michael".to_string(),
            User {
                id: "2".to_string(),
                username: "michael".to_string(),
                password: "michael".to_string(),
                email: "".to_string(),
            },
        );
        Mutex::new(map)
    };
}

/// Authenticates a user by their username and password.
///
/// # Arguments
///
/// * `username` - A string slice that holds the username of the user.
/// * `password` - A string slice that holds the password of the user.
///
/// # Returns
///
/// * `Result<User, String>` - Returns `Ok(User)` if the username and password are valid, or `Err(String)` with an error message if the authentication fails.
///
/// # Example
///
/// ```
/// let username = "admin666";
/// let password = "admin666";
/// match login_by_password(username, password).await {
///     Ok(user) => println!("User authenticated: {:?}", user),
///     Err(err) => println!("Authentication failed: {}", err),
/// }
/// ```
async fn login_by_password(username: &str, password: &str) -> Result<User, String> {
    if let Some(user) = USER_MAP.lock().unwrap().get(username) {
        if user.password != password {
            log::error!("password is invalid: {}", username);
            return Err("Username or password is invalid".to_owned());
        }
        return Ok(user.clone());
    }
    Err("Username or password is invalid".to_owned())
}

/// Checks the login state by validating the provided token.
///
/// # Arguments
///
/// * `token` - A string slice that holds the token to be validated.
///
/// # Returns
///
/// * `Result<User, String>` - Returns `Ok(User)` if the token is valid and the user is authenticated,
///   or `Err(String)` with an error message if the token is invalid or expired.
///
/// # Example
///
/// ```
/// let token = "your_jwt_token_here";
/// match check_login_state(token) {
///     Ok(user) => println!("User authenticated: {:?}", user),
///     Err(err) => println!("Authentication failed: {}", err),
/// }
/// ```
fn check_login_state(token: &str) -> Result<User, String> {
    if token.is_empty() {
        return Err("token is required".to_owned());
    }
    if let Some(tp) = check_token_exp(token) {
        if let Some(user_token) = TOKEN_MAP.lock().unwrap().get(&tp.username) {
            if user_token == token {
                return Ok(User {
                    id: tp.id,
                    username: tp.username,
                    password: "".to_owned(),
                    email: "".to_owned(),
                });
            }
        }
        return Err("token is invalid".to_owned());
    }
    Err("token is expired".to_owned())
}

/// Extracts the token from the cookies in the HTTP request.
///
/// # Arguments
///
/// * `req` - A reference to the `HttpRequest` object containing the request details.
///
/// # Returns
///
/// * `Option<String>` - Returns `Some(String)` containing the token if found, or `None` if the token is not present.
///
/// # Example
///
/// ```
/// let req = HttpRequest::default();
/// if let Some(token) = get_token_from_cookie(&req) {
///     println!("Token: {}", token);
/// } else {
///     println!("Token not found");
/// }
/// ```
fn get_token_from_cookie(req: &HttpRequest) -> Option<String> {
    let cookies = req.cookies().unwrap();
    if let Some(token) = cookies.iter().clone().find(|c| c.name() == "token") {
        return Some(token.value().to_owned());
    }
    None
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    username: String,
    password: String,
}

/// Checks the validity of the username and password based on their length.
///
/// # Arguments
///
/// * `username` - A string slice that holds the username to be checked.
/// * `password` - A string slice that holds the password to be checked.
///
/// # Returns
///
/// * `Option<String>` - Returns `Some(String)` with an error message if the username or password is invalid,
///   or `None` if both are valid.
///
/// # Example
///
/// ```
/// let username = "user123";
/// let password = "pass123";
/// if let Some(error) = check_username_password(username, password) {
///     println!("Error: {}", error);
/// } else {
///     println!("Username and password are valid");
/// }
/// ```
fn check_username_password(username: &str, password: &str) -> Option<String> {
    if username.len() < 6 || username.len() > 16 {
        return Some("username or password is invalid".to_owned());
    }
    if password.len() < 6 || password.len() > 16 {
        return Some("username or password is invalid".to_owned());
    }
    None
}

/// Handles the sign-in process for a user.
///
/// # Arguments
///
/// * `req` - The `HttpRequest` object containing the request details.
/// * `form` - A JSON payload containing the `LoginUser` details.
///
/// # Returns
///
/// * `HttpResponse` - Returns a JSON response indicating the status of the sign-in process.
///
/// # Example
///
/// ```
/// let req = HttpRequest::default();
/// let form = web::Json(LoginUser {
///     username: "admin666".to_string(),
///     password: "admin666".to_string(),
/// });
/// let response = sign_in(req, form).await;
/// assert_eq!(response.status(), StatusCode::OK);
/// ```
pub async fn sign_in(req: HttpRequest, form: web::Json<LoginUser>) -> HttpResponse {
    let method = req.method().to_string();
    if method != "POST" {
        return HttpResponse::BadRequest().body("http method is invalid");
    }
    if let Some(text) = check_username_password(&form.username, &form.password) {
        return HttpResponse::BadRequest().body(text);
    }
    let result = login_by_password(&form.username, &form.password).await;
    match result {
        Ok(user) => {
            let payload = TokenInput {
                id: user.id,
                username: user.username,
            };
            if let Some(token) = generate_token(&payload) {
                if !token.is_empty() {
                    TOKEN_MAP.lock().unwrap().insert(form.username.clone(), token.clone());
                    return HttpResponse::Ok()
                        .cookie(
                            Cookie::build("token", &token)
                                .domain("")
                                .path("/")
                                .secure(true)
                                .http_only(true)
                                .finish(),
                        )
                        .json(json!({
                          "code": 0,
                          "data": token,
                          "msg": "success",
                        }));
                }
            }
            HttpResponse::Ok().json(json!({
                "code": -1,
                "data": "",
                "msg": "user login faild, generate_token error",
            }))
        }
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    username: String,
    password: String,
    email: String,
}

/// Handles the sign-up process for a new user.
///
/// # Arguments
///
/// * `req` - The `HttpRequest` object containing the request details.
/// * `form` - A JSON payload containing the `RegisterUser` details.
///
/// # Returns
///
/// * `HttpResponse` - Returns a JSON response indicating the status of the sign-up process.
///
/// # Example
///
/// ```
/// let req = HttpRequest::default();
/// let form = web::Json(RegisterUser {
///     username: "newuser".to_string(),
///     password: "newpassword".to_string(),
///     email: "newuser@example.com".to_string(),
/// });
/// let response = sign_up(req, form).await;
/// assert_eq!(response.status(), StatusCode::OK);
/// ```
pub async fn sign_up(req: HttpRequest, form: web::Json<RegisterUser>) -> HttpResponse {
    let method = req.method().to_string();
    if method != "POST" {
        return HttpResponse::BadRequest().body("http method is invalid");
    }
    if let Some(text) = check_username_password(&form.username, &form.password) {
        return HttpResponse::BadRequest().body(text);
    }
    if form.email.len() < 5 {
        return HttpResponse::BadRequest().body("email is invalid");
    }
    match USER_MAP.lock() {
        Ok(mut user_map) => {
            if user_map.contains_key(&form.username) {
                return HttpResponse::BadRequest().body(format!("username: {} is registered", form.username));
            }
            let index = user_map.len() + 1;
            let user = User {
                id: index.to_string(),
                username: form.username.clone(),
                password: form.password.clone(),
                email: form.email.clone(),
            };
            user_map.insert(user.username.clone(), user.clone());
            return HttpResponse::Ok().json(json!({
                "code": 0,
                "msg": "sign_up success",
                "data": &user,
            }));
        }
        Err(err) => {
            log::error!("USER_MAP lock error: {:?}", err);
        }
    }
    HttpResponse::BadRequest().body("sign_up error")
}

/// Refreshes the token by validating the current token and generating a new one.
///
/// # Arguments
///
/// * `req` - The `HttpRequest` object containing the request details.
///
/// # Returns
///
/// * `HttpResponse` - Returns a JSON response with the new token if the current token is valid,
///   or an unauthorized response if the token is invalid or expired.
///
/// # Example
///
/// ```
/// let req = HttpRequest::default();
/// let response = refresh_token(req).await;
/// assert_eq!(response.status(), StatusCode::OK);
/// ```
pub async fn refresh_token(req: HttpRequest) -> HttpResponse {
    if let Some(value) = get_token_from_cookie(&req) {
        if let Some(token) = check_token_exp(&value) {
            let payload = TokenInput {
                id: token.id,
                username: token.username.clone(),
            };
            let new_token = generate_token(&payload).unwrap();
            TOKEN_MAP.lock().unwrap().insert(token.username, new_token.clone());
            return HttpResponse::Ok()
                .cookie(
                    Cookie::build("token", &new_token)
                        .domain("")
                        .path("/")
                        .secure(true)
                        .http_only(true)
                        .finish(),
                )
                .json(json!({
                    "code": 0,
                    "msg": "success",
                    "data": new_token,
                }));
        }
    }
    HttpResponse::new(StatusCode::UNAUTHORIZED)
}

/// Authenticates the user based on the token present in the HTTP request cookies.
///
/// # Arguments
///
/// * `req` - The `HttpRequest` object containing the request details.
///
/// # Returns
///
/// * `HttpResponse` - Returns a JSON response with the user data if the token is valid,
///   or an unauthorized response if the token is invalid or not present.
///
/// # Example
///
/// ```
/// let req = HttpRequest::default();
/// let response = authentication(req).await;
/// assert_eq!(response.status(), StatusCode::OK);
/// ```
pub async fn authentication(req: HttpRequest) -> HttpResponse {
    if let Some(token) = get_token_from_cookie(&req) {
        let result = check_login_state(&token);
        match result {
            Ok(user) => {
                return HttpResponse::Ok().json(json!({
                    "msg": "success",
                    "data": user,
                }));
            }
            Err(err) => {
                log::error!("check_login_state error: {}", err);
            }
        }
    }
    HttpResponse::new(StatusCode::UNAUTHORIZED)
}
