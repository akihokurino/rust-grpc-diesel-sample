use tonic::{transport::Server, Request, Response, Status};

use std::env;
use std::net::SocketAddr;
use pb::user_service_server::{UserService, UserServiceServer};

mod reflection {
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("proto_descriptor");
}
pub mod common {
    tonic::include_proto!("common");
}
pub mod pb {
    tonic::include_proto!("user");
    tonic::include_proto!("post");
}

#[derive(Debug, Default)]
pub struct UserServiceImpl {}

#[tonic::async_trait]
impl UserService for UserServiceImpl {
    async fn get_me(&self, request: Request<common::Empty>) -> Result<Response<pb::User>, Status> {
        println!("Got a request: {:?}", request);

        let reply = pb::User {
            id: "1".to_string(),
            name: format!("Hello {}!", "akiho").into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(reflection::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let port = env::var("PORT").unwrap_or("3000".to_string());

    let addr: SocketAddr = ([0, 0, 0, 0], port.parse().unwrap()).into();
    let user_service = UserServiceImpl::default();

    Server::builder()
        .add_service(reflection)
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
