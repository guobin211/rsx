use actix_web::{HttpRequest, HttpResponse, web};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ApiJsonData {
    u8: u8,
    u16: u16,
    u32: u32,
    u64: u64,
    u128: u128,
    i8: i8,
    i16: i16,
    i32: i32,
    i64: i64,
    i128: i128,
    f32: f32,
    f64: f64,
    bool: bool,
    string: String,
    array: Vec<ApiJsonData>,
}

pub async fn get() -> HttpResponse {
    let first: ApiJsonData = ApiJsonData::default();
    let data = ApiJsonData {
        u8: u8::MAX,
        u16: u16::MAX,
        u32: u32::MAX,
        u64: u64::MAX,
        u128: u128::MAX,
        i8: i8::MIN,
        i16: i16::MIN,
        i32: i32::MIN,
        i64: i64::MIN,
        i128: i128::MIN,
        f32: f32::MAX,
        f64: f64::MAX,
        bool: true,
        string: "JsonData!".to_owned(),
        array: vec![first],
    };
    HttpResponse::Ok().json(data)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostData {
    id: u32,
    value: String,
}

pub async fn post(req: HttpRequest, data: web::Json<PostData>) -> HttpResponse {
    let method = req.method().to_string();
    HttpResponse::Ok().json(json!({
        "code": 0,
        "data": data,
        "msg": format!("{} is supported.", method),
    }))
}

pub async fn un_support(req: HttpRequest) -> HttpResponse {
    let method = req.method().to_string();
    HttpResponse::BadRequest().body(format!("{} is not supported.", method))
}
