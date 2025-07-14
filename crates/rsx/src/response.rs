use crate::header::Header;
use actix_web::{
    HttpResponse,
    body::{self, BoxBody, MessageBody},
    http::{StatusCode, header},
};
use serde::Serialize;

/// 按web标准实现Response
/// https://developer.mozilla.org/zh-CN/docs/Web/API/Response
pub struct Response {
    status: StatusCode,
    headers: header::HeaderMap,
    body: BoxBody,
}

impl Response {
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

impl From<Response> for HttpResponse {
    fn from(res: Response) -> Self {
        let mut builder = HttpResponse::build(res.status);
        for (key, value) in res.headers {
            builder.insert_header((key, value));
        }
        builder.body(res.body)
    }
}
