# radix-client

## A hand crafted Rust client to interact with the Radix Gateway & Core APIs

This crate aims to implement a REST client for consuming [Radix DLT](https://www.radixdlt.com)'s [Core](https://radix-babylon-core-api.redoc.ly) and [Gateway](https://radix-babylon-gateway-api.redoc.ly) APIs. It crate supports both async and blocking use.

While the core and gateway API are similar, there are subtle differences between their endpoint names, request schemas and response schemas. That's why they're implemented separately.

# Example

```Rust
// Use either the blocking or non-blocking client from the client module
use radix_clients::GatewayClientBlocking;

// Instantiate a new client with a base URL
let client = GatewayClientBlocking::new(
    "https://mainnet.radixdlt.com".to_string(),
);

let response = client.get_state_entity_fungibles_page(
    None,
    None,
    None,
    "pool_rdx1c5shqw3yq5s7l6tr2k9h9k68cqsju9upr2wq5672rw5gt6y5s6rypj"
        .to_string(),
    None,
    None,
).unwrap();
```
