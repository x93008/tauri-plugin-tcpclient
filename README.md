# tauri-plugin-tcpclient

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app)

A Tauri plugin for TCP client connections. Send and receive TCP data directly from JavaScript.

## Features

- Connect to TCP servers from JavaScript
- Send and receive data asynchronously
- Event-based notifications for connection state changes
- Support for binding to specific local addresses
- Proper error handling including Windows disconnect detection

## Platform Support

| Platform | Supported |
| -------- | --------- |
| Linux    | ✓         |
| Windows  | ✓         |
| macOS    | ✓         |

## Installation

### Rust

Add to your `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri-plugin-tcpclient = { git = "https://github.com/x93008/tauri-plugin-tcpclient" }
```
or
```toml
[dependencies]
tauri-plugin-tcpclient = "0.2"
```

### JavaScript

```sh
npm install tauri-plugin-tcpclient
```

## Usage

### Register Plugin

```rust
// src-tauri/src/lib.rs
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_tcpclient::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### JavaScript API

```typescript
import {
  connect,
  send,
  sendString,
  disconnect,
  listenEvents,
  isConnected,
  getConnections,
  decodeMessage,
} from 'tauri-plugin-tcpclient';

// Listen for events
const unlisten = await listenEvents((event) => {
  const { id, event: e } = event.payload;

  switch (e.type) {
    case 'Connect':
      console.log(`[${id}] Connected to ${e.data.endpoint}`);
      break;
    case 'Message':
      const text = decodeMessage(e.data.data);
      console.log(`[${id}] Received: ${text}`);
      break;
    case 'Disconnect':
      console.log(`[${id}] Disconnected: ${e.data.reason.type}`);
      break;
    case 'Error':
      console.error(`[${id}] Error: ${e.data.message}`);
      break;
  }
});

// Connect to server
await connect('my-conn', '192.168.1.100:8080');

// Check connection
console.log('Connected:', await isConnected('my-conn'));

// Send string
await sendString('my-conn', 'Hello World');

// Send raw bytes
await send('my-conn', [0x01, 0x02, 0x03]);

// Get all connections
console.log('Active connections:', await getConnections());

// Disconnect
await disconnect('my-conn');

// Stop listening
unlisten();
```

## API Reference

| Function | Description |
|----------|-------------|
| `connect(id, endpoint)` | Connect to TCP server |
| `connectWithBind(id, localAddr, endpoint)` | Connect with specific local address |
| `send(id, data)` | Send bytes to server |
| `sendString(id, text)` | Send string to server |
| `disconnect(id)` | Close connection |
| `isConnected(id)` | Check if connection exists |
| `getConnections()` | Get all active connection IDs |
| `listenEvents(callback)` | Listen for TCP events |

## Events

| Event | Description |
|-------|-------------|
| `Connect` | Connection established |
| `Message` | Data received from server |
| `Disconnect` | Connection closed (with reason) |
| `Error` | Error occurred |

## License

MIT License
