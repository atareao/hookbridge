use serde_json::{json, Value};
use serde::{Serialize, Deserialize, };
use std::collections::HashMap;
use urlencoding::encode;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, debug};
use reqwest::{Client, header::{HeaderMap, HeaderValue,
    HeaderName}};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ServiceType{
    Matrix,
    ZincObserve,
    Telegram,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Service {
    service_type: ServiceType,
    config: HashMap<String, String>,
}

impl Service{
    pub async fn post(&self, message: &str) -> Result<String, String>{
        info!("post");
        match &self.service_type{
            ServiceType::Matrix => {
                self.post_with_matrix(message).await
            },
            ServiceType::ZincObserve => {
                self.post_with_zinc_observe(message).await
            },
            ServiceType::Telegram => {
                self.post_with_telegram(message).await
            },
        }
    }
    async fn post_with_zinc_observe(&self, message: &str) -> Result<String, String>{
        info!("post_with_zinc_observe");
        debug!("Post with zinc: {}", message);
        let base_url = self.config.get("url").unwrap();
        let index = self.config.get("index").unwrap();
        let token = self.config.get("token").unwrap();
        let url = format!("{}/api/default/{}/_json", base_url, index);
        info!("Url: {}", url);
        info!("Message: {}", message);
        let mut header_map = HeaderMap::new();
        header_map.append(HeaderName::from_str("Content-type").unwrap(),
                          HeaderValue::from_str("application/json").unwrap());
        header_map.append(HeaderName::from_str("Accept").unwrap(),
                          HeaderValue::from_str("application/json").unwrap());
        header_map.append(HeaderName::from_str("Authorization").unwrap(),
                          HeaderValue::from_str(&format!("Basic {}", token)).unwrap());
        info!("header_map: {:?}", &header_map);
        let body = json!([{
            "message": message,
        }]);
        Self::_post(&url, header_map, &body)
            .await
    }

    async fn post_with_matrix(&self, message: &str) -> Result<String, String>{
        info!("post_with_matrix");
        debug!("Post with matrix: {}", message);
        let token = self.config.get("token").unwrap();
        let room = encode(self.config.get("room").unwrap());
        let base_url = self.config.get("url").unwrap();
        let now = SystemTime::now();
        let ts = now.duration_since(UNIX_EPOCH).expect("Time went backwrds").as_secs();
        let url = format!("https://{}/_matrix/client/v3/rooms/{}:{}/send/m.room.message/{}", base_url, room, base_url, ts);
        let mut html = markdown::to_html(message);
        html = html[..html.len()-1].to_string();
        let body = json!({
            "msgtype": "m.text",
            "format": "org.matrix.custom.html",
            "body": message,
            "formatted_body": html
        });
        let mut header_map = HeaderMap::new();
        header_map.insert(HeaderName::from_str("Content-type").unwrap(),
                          HeaderValue::from_str("application/json").unwrap());
        header_map.append(HeaderName::from_str("Authorization").unwrap(),
                          HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());
        Self::_put(&url, header_map, &body)
            .await
    }

    async fn post_with_telegram(&self, message: &str) -> Result<String, String>{
        info!("post_with_telegram");
        debug!("Post with telegram: {}", message);
        let token = self.config.get("token").unwrap();
        let chat_id = self.config.get("chat_id").unwrap();
        let message_thread_id = self.config.get("message_thread_id").unwrap();
        let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
        let body = json!({
                "chat_id": chat_id,
                "message_thread_id": message_thread_id,
                "text": message,
        });
        let mut header_map = HeaderMap::new();
        header_map.insert(HeaderName::from_str("Content-type").unwrap(),
                          HeaderValue::from_str("application/json").unwrap());
        Self::_post(&url, header_map, &body)
            .await
    }

    async fn _post(url: &str, header_map: HeaderMap, body: &Value) -> Result<String, String>{
        let client = Client::builder()
            .default_headers(header_map)
            .build()
            .unwrap();
        let content = serde_json::to_string(body).unwrap();
        client.post(url).body(content).send()
            .await
            .map_err(|err| err.to_string())?
            .text()
            .await
            .map_err(|err| err.to_string())
    }

    async fn _put(url: &str, header_map: HeaderMap, body: &Value) -> Result<String, String>{
        let client = Client::builder()
            .default_headers(header_map)
            .build()
            .unwrap();
        let content = serde_json::to_string(body).unwrap();
        client.put(url).body(content).send()
            .await
            .map_err(|err| err.to_string())?
            .text()
            .await
            .map_err(|err| err.to_string())
    }
}
