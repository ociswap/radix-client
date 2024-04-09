# radix-client

## A hand crafted Rust client to interact with the Radix Gateway & Core APIs

This crate aims to implement a REST client for consuming [Radix DLT](https://www.radixdlt.com)'s [Core](https://radix-babylon-core-api.redoc.ly) and [Gateway](https://radix-babylon-gateway-api.redoc.ly) APIs. This crate supports both async and blocking use through a blocking and an async client struct.

While the core and gateway API are similar, there are subtle differences between their endpoint names, request schemas and response schemas. That's why they're implemented separately.

# Example

```Rust
// Use either the blocking or non-blocking client from the client module
use radix_clients::GatewayClientBlocking;

// Instantiate a new client with a base URL
let client = GatewayClientBlocking::new(
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
