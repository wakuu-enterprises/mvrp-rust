use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub struct MVRPServer {
    addr: String,
}

impl MVRPServer {
    pub async fn new(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(MVRPServer {
            addr: addr.to_string(),
        })
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.addr).await?;
        println!("MVRP server listening on {}", self.addr);

        loop {
            let (socket, _) = listener.accept().await?;
            tokio::spawn(async move {
                if let Err(e) = process(socket).await {
                    eprintln!("Failed to process connection: {}", e);
                }
            });
        }
    }
}

async fn process(mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut framed = FramedRead::new(&mut socket, LinesCodec::new());
    let request_line = framed.next_line().await?.ok_or("Failed to read request line")?;
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return Err("Malformed request line".into());
    }
    let method = parts[0];
    let url = parts[1];

    let mut headers = HashMap::new();
    while let Some(header) = framed.next_line().await? {
        if header.trim().is_empty() {
            break;
        }
        let header_parts: Vec<&str> = header.splitn(2, ": ").collect();
        if header_parts.len() == 2 {
            headers.insert(header_parts[0].to_string(), header_parts[1].to_string());
        }
    }

    let body = framed.next_line().await?.unwrap_or_default();

    println!("Received {} request for {} with body: {}", method, url, body);
    handle_request(socket, method, url, headers, body).await
}

async fn handle_request(mut socket: TcpStream, method: &str, _url: &str, _headers: HashMap<String, String>, _body: String) -> Result<(), Box<dyn std::error::Error>> {
    let (status_line, response_body) = match method {
        "OPTIONS" => ("MVRP/1.0 204 No Content", ""),
        "CREATE" => ("MVRP/1.0 201 Created", "Resource created\n"),
        "READ" => ("MVRP/1.0 200 OK", "Resource read\n"),
        "EMIT" => ("MVRP/1.0 200 OK", "Event emitted\n"),
        "BURN" => ("MVRP/1.0 200 OK", "Resource burned\n"),
        _ => ("MVRP/1.0 405 Method Not Allowed", "Method not allowed\n"),
    };

    let mut response_headers = HashMap::new();
    response_headers.insert("Content-Type".to_string(), "text/plain".to_string());
    response_headers.insert("X-Custom-Header".to_string(), encrypt("CustomHeaderValue", "secret-key"));

    let mut response = format!("{}\r\n", status_line);
    for (k, v) in &response_headers {
        response += &format!("{}: {}\r\n", k, v);
    }
    response += &format!("\r\n{}", response_body);

    socket.write_all(response.as_bytes()).await?;
    socket.shutdown(socket).await?;
    Ok(())
}

fn encrypt(text: &str, key: &str) -> String {
    let key = Sha256::digest(key.as_bytes());
    let cipher = Aes256Cbc::new_var(&key, &key[0..16]).unwrap();
    let ciphertext = cipher.encrypt_vec(text.as_bytes());
    hex::encode(ciphertext)
}
