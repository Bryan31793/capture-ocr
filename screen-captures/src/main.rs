/// Main server entry point
#[tokio::main]
async fn main() {
    screen_captures::app::run().await;
}

