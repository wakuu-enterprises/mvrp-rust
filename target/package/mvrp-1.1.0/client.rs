use std::collections::HashMap;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

pub struct MVRPClient {
    addr: String,
}

impl MVRPClient {
    pub async fn new(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(MVRPClient {
            addr: addr.to_string(),
        })
    }

    pub async fn send_request(&mut self, method: &str, url: &str, body: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut socket = TcpStream::connect(&self.addr).await?;
        let mut framed_write = FramedWrite::new(&mut socket, LinesCodec::new());
        let request_line = format!("{} {} MVRP/1.0", method, url);
        framed_write.send(request_line).await?;
        let headers = format!("Content-Length: {}", body.len());
        framed_write.send(headers).await?;
        framed_write.send("".to_string()).await?;
        framed_write.send(body.to_string()).await?;

        let mut framed_read = FramedRead::new(socket, LinesCodec::new());
        let mut response = String::new();
        while let Some(line) = framed_read.next_line().await? {
            if line.trim().is_empty() {
                break;
            }
            response += &line;
        }
        let body = framed_read.next_line().await?.unwrap_or_default();
        response += &body;

        Ok(response)
    }
}
