# radix-client

## A hand crafted Rust client to interact with the Radix Gateway & Core APIs

This crate aims to implement a REST client for consuming [Radix DLT](https://www.radixdlt.com)'s [Core](https://radix-babylon-core-api.redoc.ly) and [Gateway](https://radix-babylon-gateway-api.redoc.ly) APIs. This crate supports both async and blocking use through a blocking and an async client struct.

While it is theoretically possible to generate this client from an OpenAPI specification, our experience with the generation of clients from specifications as complex as these, is that the generated code often has issues. In addition to issues with the generation, the available client generators produce less ergonomic clients, which is what lead to this manual implementation.

The core and gateway API are similar, but there are subtle differences between their endpoint names, request schemas and response schemas. That's why they're implemented separately in this crate.

Only a subset of the available endpoints is implemented, but we will implement endpoints as we need them. Community contributions are welcomed as well.

# Blocking example

```Rust
// Use a blocking client
use radix_client::GatewayClientBlocking;

// Instantiate a new client with a base URL
let client = GatewayClientBlocking::new(
    // or use radix_client::constants::PUBLIC_GATEWAY_URL
    "https://mainnet.radixdlt.com".to_string(),
);

// Use a builder pattern to create and execute the request
let response = client
    .get_transactions_stream_builder()
    .order(Order::Asc)
    .limit_per_page(10)
    .with_raw_hex()
    .execute()
    .unwrap();
```

# Async example

```Rust
// Use an async client
use radix_client::GatewayClientAsync;

// Instantiate a new client with a base URL
let client = GatewayClientAsync::new(
    // or use radix_client::constants::PUBLIC_GATEWAY_URL
    "https://mainnet.radixdlt.com".to_string(),
);

// Use a builder pattern to create and execute the request
let response = client
    .get_transactions_stream_builder()
    .order(Order::Asc)
    .limit_per_page(10)
    .with_raw_hex()
    .execute()
    .await
    .unwrap();
```

# A note on async/blocking

At the moment, the support for asynchronous programming in Rust is a bit lacking, especially when we want to bridge the gap between async and blocking code. This crate makes use of two macros to achieve an async and a blocking client without straight-up code duplication. We define potentially asynchronous operations to be async, and the `maybe_async` (Which can remove all async/await keywords from a function) crate is then used together with the `duplicate` crate to get one blocking implementation and one asynchronous one. Rust seems hard at work on something called the [Keyword Generics Initiative](https://blog.rust-lang.org/inside-rust/2022/07/27/keyword-generics.html), which should fix this issue. Until then, this is a solution.
