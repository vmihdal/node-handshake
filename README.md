## P2P Handshake

Command line tool for making p2p handshake with a bitcoin node.

## How to use

### Help messages

```
cargo run help
```

Handshake options:

```
cargo run bitcoin --help
```

### Handshake with bitcoin node

Default settings:

```
cargo run bitcoin 85.215.75.210:8333
```

With options:

```
cargo run bitcoin 85.215.75.210:8333 --start-height=0 --network=bitcoin --services=0 --timeout=5000
```

### Example output

```
handshake_cli bitcoin 85.215.75.210:8333 --start-height=1

2024-05-01T19:45:14.383518Z  Local node: Node {
  services: ServiceFlags(
      0,
  ),
  start_height: 1,
  network: Bitcoin,
}
  in handshake with peer: "85.215.75.210:8333"

2024-05-01T19:45:14.409787Z  direction: "OUT", Version(
  VersionMessage {
      version: 70001,
      services: ServiceFlags(
          0,
      ),
      timestamp: 1714592714,
      receiver: Address {services: ServiceFlags(NONE), address: 85.215.75.210, port: 8333},
      sender: Address {services: ServiceFlags(NONE), address: 0.0.0.0, port: 47640},
      nonce: 1714592714,
      user_agent: "",
      start_height: 1,
      relay: false,
  },
)
  in handshake with peer: "85.215.75.210:8333"

2024-05-01T19:45:14.438073Z  direction: "IN", Version(
  VersionMessage {
      version: 70016,
      services: ServiceFlags(
          1097,
      ),
      timestamp: 1714592714,
      receiver: Address {services: ServiceFlags(NONE), address: 178.37.240.179, port: 47640},
      sender: Address {services: ServiceFlags(NETWORK|WITNESS|COMPACT_FILTERS|NETWORK_LIMITED), address: 0.0.0.0, port: 0},
      nonce: 6477430099363440048,
      user_agent: "/Satoshi:26.0.0/",
      start_height: 841679,
      relay: true,
  },
)
  in handshake with peer: "85.215.75.210:8333"

2024-05-01T19:45:14.438116Z  direction: "OUT", Verack
  in handshake with peer: "85.215.75.210:8333"

2024-05-01T19:45:14.438126Z  direction: "IN", Verack
  in handshake with peer: "85.215.75.210:8333"

2024-05-01T19:45:14.438145Z  close, time.busy: 296µs, time.idle: 54.3ms
  in handshake with peer: "85.215.75.210:8333"
```