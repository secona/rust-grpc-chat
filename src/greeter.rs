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
        GreeterServer::new(Self)
    }
}

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        let input = request.get_ref();

        let row = sqlx::query!("SELECT $1::INTEGER as num;", 1i32)
            .fetch_one(get_db_pool())
            .await
            .unwrap();

        let response = HelloResponse {
            message: format!("Hello, {} {:?}!", input.name, row.num),
        };

        Ok(Response::new(response))
    }
}
