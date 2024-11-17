use tonic::transport::Server;
use rust_grpc_chat::db::init_db_pool;
use rust_grpc_chat::greeter;
use rust_grpc_chat::chat;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_db_pool().await?;
    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(greeter::GreeterService::new())
        .add_service(chat::ChatService::new())
        .serve(addr)
        .await?;

    Ok(())
}
