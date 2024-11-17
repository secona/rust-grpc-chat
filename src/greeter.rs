use proto::greeter_server::{Greeter, GreeterServer};
use proto::{HelloRequest, HelloResponse};
use tonic::{Request, Response, Status};

use crate::db::get_db_pool;

pub mod proto {
    tonic::include_proto!("greeter");
}

#[derive(Debug, Default)]
pub struct GreeterService;

impl GreeterService {
    pub fn new() -> GreeterServer<Self> {
        GreeterServer::new(Self::default())
    }
}

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        let input = request.get_ref();

        let row: (i64,) = sqlx::query_as("SELECT $1;")
            .bind(1i64)
            .fetch_one(get_db_pool())
            .await
            .unwrap();

        let response = HelloResponse {
            message: format!("Hello, {} {:?}!", input.name, row.0),
        };

        Ok(Response::new(response))
    }
}
