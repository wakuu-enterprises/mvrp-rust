# Muvor Protocol (MVRP)

## Description

A custom protocol implementation for Muvor Protocol (MVRP) with modern features.

## Installation

```bash
cargo build
```

## Client

```bash
use mvrp::client::MVRPClient;

#[tokio::main]
async fn main() {
    let mut client = MVRPClient::new("127.0.0.1:8080").await.unwrap();
    let response = client.send_request("CREATE", "/", "Hello, server!").await.unwrap();
    println!("Response: {}", response);
}
```

## Video

```bash
use mvrp::mvvp::process_segments;

fn main() {
    process_segments("/path/to/uploads", "/path/to/structured").unwrap();
}
```

## Server

 ```bash
 use mvrp::mvrp::MVRPServer;

#[tokio::main]
async fn main() {
    let server = MVRPServer::new("127.0.0.1:8080").await.unwrap();
    server.run().await.unwrap();
}
```