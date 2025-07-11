use actix_web::http::header::CONTENT_TYPE;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// post form-data
pub async fn post_form_data(req: HttpRequest) -> HttpResponse {
    let content_type = req.headers().get(CONTENT_TYPE);
    match content_type {
        Some(content_type) => {
            let content_type_str = content_type.to_str().unwrap_or_default();
            if !content_type_str.starts_with("multipart/form-data") {
                return HttpResponse::BadRequest().body("content-type is not multipart/form-data");
            }
            HttpResponse::Ok().json(json!({
                "code": 0,
                "data": "post",
                "msg": "multipart/form-data has not implemented yet",
            }))
        }
        None => HttpResponse::BadRequest().body("Content-Type header is missing"),
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UrlFormData {
    id: String,
    value: String,
    fact: String,
}

/// post form-urlencoded
pub async fn post_form_url(req: HttpRequest, form: web::Form<UrlFormData>) -> HttpResponse {
    let content_type = req.headers().get("content-type").unwrap();
    if content_type != "application/x-www-form-urlencoded" {
        return HttpResponse::BadRequest().body("content-type is not application/x-www-form-urlencoded");
    }
    HttpResponse::Ok().json(json!({
        "code": 0,
        "method": "post",
        "id": form.id.clone(),
        "value": form.value.clone(),
        "fact": form.fact.clone(),
    }))
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JsonFormData {
    name: String,
    age: i32,
}

/// post json
pub async fn post_form_json(req: HttpRequest, data: web::Json<JsonFormData>) -> HttpResponse {
    let content_type = req.headers().get("content-type").unwrap();
    if content_type != "application/json" {
        return HttpResponse::BadRequest().body("content-type is not application/json");
    }
    let name = data.name.clone();
    let age = data.age;
    HttpResponse::Ok().json(json!({
        "code": 0,
        "method": "post",
        "msg": format!("name: {}, age: {}", name, age),
    }))
}
