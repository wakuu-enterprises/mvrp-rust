mod mvrp;
mod mvvp;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        // Start the server
        mvrp::MVRPServer::new("127.0.0.1:8443", "path/to/server-key.pem", "path/to/server-cert.pem").await;
    });

    // Periodically process video segments
    mvvp::process_segments("/path/to/uploads", "/path/to/structured").unwrap();
}