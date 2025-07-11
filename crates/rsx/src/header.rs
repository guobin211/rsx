use actix_web::{Error, FromRequest, HttpRequest, http as actix_http};
use http::{
    HeaderMap,
    header::{HeaderName, HeaderValue},
};
use serde_json;
use std::collections::HashMap;
use std::fmt;
use std::future::{Ready, ready};
use std::str::FromStr;

/// 按web标准实现Header
#[derive(Debug, Clone)]
pub struct Header(HeaderMap);

impl Header {
    /// 创建一个新的空的Header
    pub fn new() -> Self {
        Header(HeaderMap::new())
    }

    /// 从现有的HeaderMap创建Header
    pub fn from_headers(headers: HeaderMap) -> Self {
        Header(headers)
    }

    /// 从HashMap创建Header
    pub fn from_map(map: &HashMap<String, String>) -> Self {
        let mut headers = HeaderMap::new();
        for (name, value) in map.iter() {
            if let (Ok(name), Ok(value)) =
                (HeaderName::from_str(name), HeaderValue::from_str(value))
            {
                headers.insert(name, value);
            }
        }
        Header(headers)
    }

    /// 将Header转换回http::HeaderMap
    pub fn into_header_map(self) -> HeaderMap {
        self.0
    }

    /// 追加一个header
    pub fn append(&mut self, name: &str, value: &str) {
        let name = HeaderName::from_str(name).unwrap();
        let value = HeaderValue::from_str(value).unwrap();
        self.0.append(name, value);
    }

    /// 删除一个header
    pub fn delete(&mut self, name: &str) {
        let name = HeaderName::from_str(name).unwrap();
        self.0.remove(name);
    }

    /// 获取一个header
    pub fn get(&self, name: &str) -> Option<&str> {
        let name = HeaderName::from_str(name).unwrap();
        self.0.get(name).and_then(|value| value.to_str().ok())
    }

    /// 判断一个header是否存在
    pub fn has(&self, name: &str) -> bool {
        let name = HeaderName::from_str(name).unwrap();
        self.0.contains_key(name)
    }

    /// 设置一个header
    pub fn set(&mut self, name: &str, value: &str) {
        let name = HeaderName::from_str(name).unwrap();
        let value = HeaderValue::from_str(value).unwrap();
        self.0.insert(name, value);
    }

    /// 返回一个迭代器
    pub fn entries(&self) -> impl Iterator<Item = (&str, &str)> {
        self.0
            .iter()
            .map(|(name, value)| (name.as_str(), value.to_str().unwrap_or_default()))
    }

    /// 返回json格式的字符串
    pub fn to_json(&self) -> String {
        let map: HashMap<String, &str> = self.entries().map(|(k, v)| (k.to_string(), v)).collect();
        serde_json::to_string(&map).unwrap_or_else(|_| "{}".to_string())
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (name, value) in self.0.iter() {
            write!(
                f,
                "{}: {}\r\n",
                name.as_str(),
                value.to_str().unwrap_or_default()
            )?;
        }
        Ok(())
    }
}

impl Default for Header {
    fn default() -> Self {
        Self::new()
    }
}

impl From<actix_http::header::HeaderMap> for Header {
    fn from(headers: actix_http::header::HeaderMap) -> Self {
        let mut new_headers = HeaderMap::new();
        for (name, value) in headers.iter() {
            if let Ok(value_str) = value.to_str() {
                if let (Ok(key), Ok(value)) = (
                    HeaderName::from_str(name.as_str()),
                    HeaderValue::from_str(value_str),
                ) {
                    new_headers.insert(key, value);
                }
            }
        }
        Header(new_headers)
    }
}

impl From<Header> for actix_http::header::HeaderMap {
    fn from(header: Header) -> Self {
        let mut new_headers = actix_http::header::HeaderMap::new();
        for (name, value) in header.0.iter() {
            if let Ok(value_str) = value.to_str() {
                if let (Ok(key), Ok(value)) = (
                    actix_web::http::header::HeaderName::from_str(name.as_str()),
                    actix_web::http::header::HeaderValue::from_str(value_str),
                ) {
                    new_headers.insert(key, value);
                }
            }
        }
        new_headers
    }
}

impl FromRequest for Header {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let headers = req.headers().clone();
        ready(Ok(Header::from(headers)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::header::{AUTHORIZATION, CONTENT_TYPE};

    #[test]
    fn convert_actix_header_map_to_header() {
        let mut actix_headers = actix_http::header::HeaderMap::new();
        actix_headers.insert(
            CONTENT_TYPE,
            actix_http::header::HeaderValue::from_static("application/json"),
        );
        let header = Header::from(actix_headers);
        println!("{header:?}");
        assert_eq!(header.get("Content-Type"), Some("application/json"));
    }

    #[test]
    fn convert_empty_actix_header_map_to_header() {
        let actix_headers = actix_http::header::HeaderMap::new();
        let header = Header::from(actix_headers);
        assert!(header.entries().next().is_none());
    }

    #[test]
    fn convert_header_to_actix_header_map() {
        let mut header = Header::new();
        header.append("Authorization", "Bearer token");
        let actix_headers: actix_http::header::HeaderMap = header.into();
        assert_eq!(
            actix_headers
                .get("Authorization")
                .unwrap()
                .to_str()
                .unwrap(),
            "Bearer token"
        );
    }

    #[test]
    fn convert_empty_header_to_actix_header_map() {
        let header = Header::new();
        let actix_headers: actix_http::header::HeaderMap = header.into();
        assert!(actix_headers.is_empty());
    }

    #[test]
    fn handle_invalid_header_name_in_actix_conversion() {
        let mut actix_headers = actix_http::header::HeaderMap::new();
        actix_headers.insert(
            CONTENT_TYPE,
            actix_http::header::HeaderValue::from_static("value"),
        );
        let header = Header::from(actix_headers);
        println!("{header:?}");
        assert!(header.get("Invalid-Header").is_none());
    }

    #[test]
    fn handle_invalid_header_value_in_actix_conversion() {
        let mut actix_headers = actix_http::header::HeaderMap::new();
        actix_headers.insert(
            AUTHORIZATION,
            actix_http::header::HeaderValue::from_bytes(b"\xFF").unwrap(),
        );
        let header = Header::from(actix_headers);
        assert!(header.entries().next().is_none());
    }

    #[test]
    fn test_json() {
        let mut header = Header::new();
        header.append("Content-Type", "application/json");
        header.append("Authorization", "Bearer token");
        let json = header.to_json();
        println!("{json}");
        assert!(json.contains("application/json"));
    }
}
