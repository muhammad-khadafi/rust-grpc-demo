use tonic::{transport::Server, Request, Response, Status};

use hello::greetings_server::{Greetings, GreetingsServer};
use hello::{MessageResponse, ParamRequest};

pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Debug, Default)]
pub struct GreetingsService {}

#[tonic::async_trait]
impl Greetings for GreetingsService {
    async fn say_hello(
        &self,
        request: Request<ParamRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = MessageResponse {
            message: format!("Hello {}.", req.name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50052".parse()?;
    let greetings_service = GreetingsService::default();

    Server::builder()
        .add_service(GreetingsServer::new(greetings_service))
        .serve(addr)
        .await?;

    Ok(())
}