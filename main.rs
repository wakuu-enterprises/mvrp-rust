mod mvrp;
mod mvvp;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        // Start the server
        mvrp::MVRPServer::new("127.0.0.1:8443").await;
    });

    // Periodically process video segments
    mvvp::process_segments("/uploads", "/structured").unwrap();
}