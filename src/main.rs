mod server;

extern crate log;
extern crate pretty_env_logger;

#[tokio::main]
async fn main() {
    // Init logger
    pretty_env_logger::init();

    // Start the API server to handle requests
    server::start( ([127,0,0,1], 3000) ).await;
}
