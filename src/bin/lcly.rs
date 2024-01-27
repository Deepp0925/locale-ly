/// This is the main entry point for the CLI application.
#[tokio::main]
async fn main() {
    cli::run().await;
}
