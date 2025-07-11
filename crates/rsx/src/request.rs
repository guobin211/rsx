use crate::header::Header;
use actix_web::http::{Method, Uri};
use actix_web::{Error, FromRequest, HttpRequest, dev::Payload, web::Bytes};
use futures_util::future::LocalBoxFuture;
use serde::de::DeserializeOwned;
use serde_json;

/// 按web标准实现Request
/// https://developer.mozilla.org/zh-CN/docs/Web/API/Request
#[derive(Debug, Clone)]
pub struct Request {
    method: Method,
    url: Uri,
    headers: Header,
    body: Bytes,
    body_used: bool,
}

impl Request {
    /// 获取请求方法
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// 获取请求的URL
    pub fn url(&self) -> String {
        self.url.to_string()
    }

    /// 获取请求头
    pub fn headers(&self) -> &Header {
        &self.headers
    }

    /// 检查URL是否有效
    pub fn is_invalid_url(&self) -> bool {
        self.url.to_string().starts_with("http://") || self.url.to_string().starts_with("https://")
    }

    /// 获取请求体是否已被使用
    pub fn body_used(&self) -> bool {
        self.body_used
    }

    /// 以文本形式消费请求体
    pub async fn text(mut self) -> Result<String, Error> {
        if self.body_used {
            return Err(Error::from(std::io::Error::other("Body already used")));
        }
        self.body_used = true;
        String::from_utf8(self.body.to_vec())
            .map_err(|e| Error::from(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))
    }

    /// 以JSON形式消费请求体
    pub async fn json<T: DeserializeOwned>(mut self) -> Result<T, Error> {
        if self.body_used {
            return Err(Error::from(std::io::Error::other("Body already used")));
        }
        self.body_used = true;
        serde_json::from_slice(&self.body)
            .map_err(|e| Error::from(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))
    }

    /// 以字节形式消费请求体
    pub async fn bytes(mut self) -> Result<Bytes, Error> {
        if self.body_used {
            return Err(Error::from(std::io::Error::other("Body already used")));
        }
        self.body_used = true;
        Ok(self.body.clone())
    }
}

impl FromRequest for Request {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let req_clone = req.clone();
        let mut payload_clone = payload.take();

        Box::pin(async move {
            let method = req_clone.method().clone();
            let url = req_clone.uri().to_owned();
            let headers = Header::from(req_clone.headers().clone());
            let body = Bytes::from_request(&req_clone, &mut payload_clone).await?;
            Ok(Request {
                method,
                url,
                headers,
                body,
                body_used: false,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::header;
    use actix_web::test::TestRequest;
    use serde_json::json;

    #[actix_rt::test]
    async fn create_request_from_http_request() {
        let req = TestRequest::default()
            .method(Method::POST)
            .uri("http://localhost/test")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .set_payload(Bytes::from_static(b"{\"key\":\"value\"}"))
            .to_http_request();

        let mut payload = Payload::None;
        let request = Request::from_request(&req, &mut payload).await.unwrap();
        assert_eq!(request.method(), &Method::POST);
        assert_eq!(request.url(), "http://localhost/test");
        assert_eq!(
            request.headers().get("Content-Type"),
            Some("application/json")
        );
        assert!(!request.body_used());
    }

    #[actix_rt::test]
    async fn consume_request_body_as_text() {
        let req = TestRequest::default()
            .set_payload(Bytes::from_static(b"Hello, world!"))
            .to_http_request();

        let mut payload = Payload::None;
        let request = Request::from_request(&req, &mut payload).await.unwrap();

        let text = request.text().await.unwrap();
        assert_eq!(text, "Hello, world!");
    }

    #[actix_rt::test]
    async fn consume_request_body_as_json() {
        let req = TestRequest::default()
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .set_payload(Bytes::from_static(b"{\"key\":\"value\"}"))
            .to_http_request();

        let mut payload = Payload::None;
        let request = Request::from_request(&req, &mut payload).await.unwrap();

        let json: serde_json::Value = request.json().await.unwrap();
        assert_eq!(json, json!({"key": "value"}));
    }

    #[actix_rt::test]
    async fn consume_request_body_as_bytes() {
        let req = TestRequest::default()
            .set_payload(Bytes::from_static(b"binary data"))
            .to_http_request();

        let mut payload = Payload::None;
        let request = Request::from_request(&req, &mut payload).await.unwrap();

        let bytes = request.bytes().await.unwrap();
        assert_eq!(bytes, Bytes::from_static(b"binary data"));
    }

    #[actix_rt::test]
    async fn error_when_invalid_json_body() {
        let req = TestRequest::default()
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .set_payload(Bytes::from_static(b"invalid json"))
            .to_http_request();

        let mut payload = Payload::None;
        let request = Request::from_request(&req, &mut payload).await.unwrap();

        let result: Result<serde_json::Value, _> = request.json().await;
        assert!(result.is_err());
    }

    #[actix_rt::test]
    async fn error_when_invalid_url() {
        let req = TestRequest::default().uri("invalid_url").to_http_request();

        let mut payload = Payload::None;
        let result = Request::from_request(&req, &mut payload).await;
        assert!(result.is_ok());
    }
}
