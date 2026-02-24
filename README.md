# valve-rcon

[![Crates.io](https://img.shields.io/crates/v/valve-rcon)](https://crates.io/crates/valve-rcon)
[![Downloads](https://img.shields.io/crates/d/valve-rcon)](https://crates.io/crates/valve-rcon)
[![License: MIT](https://img.shields.io/crates/l/valve-rcon)](https://opensource.org/licenses/MIT)
[![Rust Edition](https://img.shields.io/badge/rust%20edition-2021-orange)](https://doc.rust-lang.org/edition-guide/rust-2021/)

A zero-dependency Rust implementation of [Valve's RCON protocol](https://developer.valvesoftware.com/wiki/Source_RCON_Protocol), used to send remote console commands to Source engine game servers (CS2, TF2, Garry's Mod, etc.).

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
valve-rcon = "0.1.2"
```

## Usage

```rust
use valve_rcon::prelude::*;

// Build an auth packet from a password
let auth = Packet::new(PacketType::Auth, b"my_password".to_vec());

// Serialize to bytes for sending over a TCP socket
let bytes: Vec<u8> = auth.into();

// Parse a raw response from the server
let response = Packet::try_from(&bytes).expect("valid packet");
println!("Type: {:?}, Body: {:?}", response.packet_type, response.body);

// Convenience: build an ExecCommand packet from a string
let cmd = Packet::from("status");
```

## API

### `Packet`

| Field | Type | Description |
|---|---|---|
| `size` | `i32` | Packet size in bytes (excluding the size field itself) |
| `id` | `i32` | Client-chosen request ID echoed back in responses |
| `packet_type` | `PacketType` | One of the four RCON packet types |
| `body` | `Vec<u8>` | Packet payload |

**Constructors:**
- `Packet::new(packet_type, body)` — create a packet with a given type and body
- `Packet::from("command")` — create an `ExecCommand` packet from a string slice
- `Packet::try_from(&vec)` — deserialize a packet from raw bytes; returns `Err(BufferParseError)` on malformed input

**Serialization:**
- `let bytes: Vec<u8> = packet.into()` — serialize to a little-endian byte buffer ready to send

### `PacketType`

| Variant | Wire value | Direction |
|---|---|---|
| `Auth` | `3` | Client → Server |
| `AuthResponse` | `2` | Server → Client |
| `ExecCommand` | `2` | Client → Server |
| `ResponseValue` | `0` | Server → Client |

### `BufferParseError`

Returned by `Packet::try_from` when the byte buffer is too short or contains an unrecognized packet type. Implements `std::error::Error` and `Display`.

## License

MIT
