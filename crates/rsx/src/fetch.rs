use anyhow::Result;
use reqwest::{Client, Method, header};

use crate::response::{ClientResponse, Response};

/// 按web标准实现Fetch和fetch函数
/// https://developer.mozilla.org/zh-CN/docs/Web/API/Fetch_API
pub struct Fetch;

/// fetch请求的选项
#[derive(Default)]
pub struct FetchOptions {
    pub method: Option<Method>,
    pub headers: Option<header::HeaderMap>,
    pub body: Option<String>,
}

/// 执行fetch请求
pub async fn fetch(url: &str, options: Option<FetchOptions>) -> Result<Response> {
    let client = Client::new();
    let method = options
        .as_ref()
        .and_then(|o| o.method.clone())
        .unwrap_or(Method::GET);
    let mut request_builder = client.request(method, url);

    if let Some(opts) = options {
        if let Some(headers) = opts.headers {
            request_builder = request_builder.headers(headers);
        }
        if let Some(body) = opts.body {
            request_builder = request_builder.body(body);
        }
    }

    let response = request_builder.send().await?;
    Ok(Response::Client(ClientResponse { raw: response }))
}
