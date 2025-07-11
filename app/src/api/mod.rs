#![allow(dead_code)]

use actix_web::{Scope, web};

mod auth;
mod form;
mod json;
mod token;

pub fn router_scope() -> Scope {
    web::scope("/api")
        .service(
            web::resource("/json")
                .route(web::get().to(json::get))
                .route(web::post().to(json::post))
                .route(web::put().to(json::un_support))
                .route(web::delete().to(json::un_support)),
        )
        .service(web::resource("/form/form-data").route(web::post().to(form::post_form_data)))
        .service(web::resource("/form/form-urlencoded").route(web::post().to(form::post_form_url)))
        .service(web::resource("/form/json").route(web::post().to(form::post_form_json)))
        .service(web::resource("/auth/sign_in").route(web::post().to(auth::sign_in)))
        .service(web::resource("/auth/sign_up").route(web::post().to(auth::sign_up)))
        .service(web::resource("/auth/refresh_token").route(web::post().to(auth::refresh_token)))
        .service(
            web::resource("/auth/check_login")
                .route(web::post().to(auth::authentication))
                .route(web::get().to(auth::authentication))
                .route(web::put().to(auth::authentication))
                .route(web::delete().to(auth::authentication)),
        )
}
