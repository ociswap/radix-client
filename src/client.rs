//! # Client
//!
//! This module contains client structs for interacting with the Gateway and Core APIs.
//! This only contains the structs and some low-level methods like a post request.
//! Further impls on these structs may be provided by the modules responsible
//! for the specific API endpoints.

#[cfg(feature = "gateway")]
pub mod gateway {
    use duplicate::duplicate_item;
    use log::trace;
    use maybe_async::{must_be_async, must_be_sync};
    use serde::Serialize;
    use std::rc::Rc;
    use std::sync::Arc;

    #[duplicate_item(
        client_type                reqwest_client_type             smart_pointer;
        [ GatewayClientAsync ]        [ reqwest::Client ]             [ Arc ];
        [ GatewayClientBlocking ]     [ reqwest::blocking::Client ]   [ Rc];
    )]
    #[derive(Debug, Clone)]
    pub struct client_type {
        pub base_url: String,
        pub client: smart_pointer<reqwest_client_type>,
    }

    #[duplicate_item(
        client_type                reqwest_client_type           maybe_async_attr    smart_pointer;
        [ GatewayClientAsync ]     [ reqwest::Client ]           [ must_be_async ]   [ Arc ];
        [ GatewayClientBlocking ]  [ reqwest::blocking::Client ] [ must_be_sync ]    [ Rc ];
    )]
    impl client_type {
        pub fn new(base_url: String) -> client_type {
            client_type {
                base_url,
                client: smart_pointer::new(reqwest_client_type::new()),
            }
        }

        #[maybe_async_attr]
        pub async fn post<S: Serialize>(
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
            trace!("Status: {status} - Response: {text}");
            Ok((text, status))
        }
    }
}

#[cfg(feature = "core")]
pub mod core {
    use duplicate::duplicate_item;
    use log::trace;
    use maybe_async::{must_be_async, must_be_sync};
    use serde::Serialize;
    use std::rc::Rc;
    use std::sync::Arc;

    #[duplicate_item(
    client_type                reqwest_client_type             smart_pointer;
    [ CoreClientAsync ]        [ reqwest::Client ]             [ Arc ];
    [ CoreClientBlocking ]     [ reqwest::blocking::Client ]   [ Rc];
)]
    #[derive(Debug, Clone)]
    pub struct client_type {
        pub base_url: String,
        pub client: smart_pointer<reqwest_client_type>,
    }

    #[duplicate_item(
    client_type                reqwest_client_type           maybe_async_attr    smart_pointer;
    [ CoreClientAsync ]     [ reqwest::Client ]           [ must_be_async ]   [ Arc ];
    [ CoreClientBlocking ]  [ reqwest::blocking::Client ] [ must_be_sync ]    [ Rc ];
)]
    impl client_type {
        pub fn new(base_url: String) -> client_type {
            client_type {
                base_url,
                client: smart_pointer::new(reqwest_client_type::new()),
            }
        }

        #[maybe_async_attr]
        pub async fn post<S: Serialize>(
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
            trace!("Status: {status} - Response: {text}");
            Ok((text, status))
        }
    }
}
