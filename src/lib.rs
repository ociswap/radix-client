pub mod constants;
#[cfg(feature = "core")]
pub mod core;
#[cfg(feature = "gateway")]
pub mod gateway;

use duplicate::duplicate_item;
use maybe_async::{must_be_async, must_be_sync};
use serde::Serialize;

#[cfg(feature = "gateway")]
#[duplicate_item(
    client_type                 reqwest_client_type ;
    [ GatewayClientAsync ]     [ reqwest::Client ];
    [ GatewayClientBlocking ]  [ reqwest::blocking::Client ];

  )]
#[derive(Debug, Clone)]
pub struct client_type {
    pub base_url: String,
    pub client: reqwest_client_type,
}

#[cfg(feature = "core")]
#[duplicate_item(
    client_type                 reqwest_client_type ;
    [ CoreClientAsync ]        [ reqwest::Client ];
    [ CoreClientBlocking ]     [ reqwest::blocking::Client ];
  )]
#[derive(Debug, Clone)]
pub struct client_type {
    pub base_url: String,
    pub client: reqwest_client_type,
}

#[cfg(feature = "gateway")]
#[duplicate_item(
    client_type                 reqwest_client_type           maybe_async_attr;
    [ GatewayClientAsync ]     [ reqwest::Client ]          [ must_be_async ];
    [ GatewayClientBlocking ]  [ reqwest::blocking::Client ] [ must_be_sync ];
  )]
impl client_type {
    pub fn new(base_url: String) -> client_type {
        client_type {
            base_url,
            client: reqwest_client_type::new(),
        }
    }

    #[maybe_async_attr]
    pub async fn post_request<S: Serialize>(
        &self,
        path: &str,
        body: S,
    ) -> Result<(String, reqwest::StatusCode), reqwest::Error> {
        let res = self
            .client
            .post(format!("{}/{}", &self.base_url, path))
            .header(reqwest::header::ACCEPT, "application/json")
            .header(reqwest::header::USER_AGENT, "reqwest/0.11.0")
            .json(&body)
            .send()
            .await
            .unwrap();
        let status = res.status();
        let text = res.text().await?;
        Ok((text, status))
    }
}

#[cfg(feature = "core")]
#[duplicate_item(
    client_type                 reqwest_client_type           maybe_async_attr;
    [ CoreClientAsync ]        [ reqwest::Client ]          [ must_be_async ];
    [ CoreClientBlocking ]     [ reqwest::blocking::Client ] [ must_be_sync ];
)]
impl client_type {
    pub fn new(base_url: String) -> client_type {
        client_type {
            base_url,
            client: reqwest_client_type::new(),
        }
    }

    #[maybe_async_attr]
    pub async fn post_request<S: Serialize>(
        &self,
        path: &str,
        body: S,
    ) -> Result<(String, reqwest::StatusCode), reqwest::Error> {
        let res = self
            .client
            .post(format!("{}/{}", &self.base_url, path))
            .header(reqwest::header::ACCEPT, "application/json")
            .header(reqwest::header::USER_AGENT, "reqwest/0.11.0")
            .json(&body)
            .send()
            .await?;
        let status = res.status();
        let text = res.text().await?;
        Ok((text, status))
    }
}
