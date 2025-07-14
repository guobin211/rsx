use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{
    HttpResponse,
    body::{self, BoxBody, MessageBody},
    http::{StatusCode, header},
};
use anyhow::Result;
use http::HeaderMap;
use reqwest::Response as ReqwestResponse;
use serde::{Serialize, de::DeserializeOwned};
use std::str::FromStr;

use crate::header::Header;

/// 客户端响应
pub struct ClientResponse {
    pub raw: ReqwestResponse,
}

impl ClientResponse {
    /// 获取响应状态码
    pub fn status(&self) -> u16 {
        self.raw.status().as_u16()
    }

    /// 获取响应头
    pub fn headers(&self) -> &HeaderMap {
        self.raw.headers()
    }

    /// 以文本形式获取响应体
    pub async fn text(self) -> Result<String> {
        Ok(self.raw.text().await?)
    }

    /// 以JSON形式获取响应体
    pub async fn json<T: DeserializeOwned>(self) -> Result<T> {
        Ok(self.raw.json::<T>().await?)
    }

    /// 以字节形式获取响应体
    pub async fn bytes(self) -> Result<bytes::Bytes> {
        Ok(self.raw.bytes().await?)
    }
}

/// 服务端响应
pub struct ServerResponse {
    status: StatusCode,
    headers: header::HeaderMap,
    body: BoxBody,
}

impl ServerResponse {
    /// 创建一个新的 Response 实例
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            headers: header::HeaderMap::new(),
            body: body::None::new().boxed(),
        }
    }

    /// 设置响应状态码
    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    /// 插入响应头
    pub fn header(mut self, key: header::HeaderName, value: header::HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// 附加响应头
    pub fn append_header(mut self, key: header::HeaderName, value: header::HeaderValue) -> Self {
        self.headers.append(key, value);
        self
    }

    /// 获取响应头
    pub fn get_header(&self, key: &header::HeaderName) -> Option<&header::HeaderValue> {
        self.headers.get(key)
    }

    /// 检查是否存在响应头
    pub fn has_header(&self, key: &header::HeaderName) -> bool {
        self.headers.contains_key(key)
    }

    /// 移除响应头
    pub fn remove_header(&mut self, key: &header::HeaderName) {
        self.headers.remove(key);
    }

    /// 设置响应头
    pub fn headers(mut self, headers: Header) -> Self {
        self.headers = headers.into();
        self
    }

    /// 发送响应头
    pub fn write_head(mut self, status: StatusCode, headers: header::HeaderMap) -> Self {
        self.status = status;
        self.headers = headers;
        self
    }

    /// 设置响应体
    pub fn body(mut self, body: impl Into<BoxBody>) -> Self {
        self.body = body.into();
        self
    }

    /// 创建一个纯文本响应
    pub fn text(body: String) -> Self {
        let mut res = Self::new(StatusCode::OK);
        res.headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("text/plain; charset=utf-8"),
        );
        res.body(BoxBody::new(body))
    }

    /// 创建一个 JSON 响应
    pub fn json<T: Serialize>(value: T) -> Result<Self, serde_json::Error> {
        let body = serde_json::to_string(&value)?;
        let mut res = Self::text(body);
        res.headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        Ok(res)
    }
}

/// 按web标准实现Response
/// https://developer.mozilla.org/zh-CN/docs/Web/API/Response
pub enum Response {
    Server(ServerResponse),
    Client(ClientResponse),
}

impl From<ServerResponse> for Response {
    fn from(res: ServerResponse) -> Self {
        Response::Server(res)
    }
}

impl From<ClientResponse> for Response {
    fn from(res: ClientResponse) -> Self {
        Response::Client(res)
    }
}

impl From<Response> for HttpResponse {
    fn from(res: Response) -> Self {
        match res {
            Response::Server(server_res) => {
                let mut builder = HttpResponse::build(server_res.status);
                for (key, value) in server_res.headers {
                    builder.insert_header((key, value));
                }
                builder.body(server_res.body)
            }
            Response::Client(client_res) => {
                let mut builder =
                    HttpResponse::build(StatusCode::from_u16(client_res.status()).unwrap());
                for (name, value) in client_res.raw.headers() {
                    let key = HeaderName::from_str(name.as_str()).unwrap();
                    let value = HeaderValue::from_str(value.to_str().unwrap()).unwrap();
                    builder.insert_header((key, value));
                }
                builder.streaming(client_res.raw.bytes_stream())
            }
        }
    }
}
