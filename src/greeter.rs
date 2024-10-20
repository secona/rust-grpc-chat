use proto::greeter_server::{Greeter, GreeterServer};
use proto::{HelloRequest, HelloResponse};
use tonic::{Request, Response, Status};

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

        let response = HelloResponse {
            message: format!("Hello, {}!", input.name),
        };

        Ok(Response::new(response))
    }
}
