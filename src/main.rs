mod errors;
mod external;
mod rpc;

use tonic::transport::Server;

use crate::external::Client;
use crate::rpc::*;
use std::env;
use std::net::SocketAddr;

mod reflection {
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("proto_descriptor");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(reflection::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let port = env::var("PORT").unwrap_or("3000".to_string());
    let addr: SocketAddr = ([0, 0, 0, 0], port.parse().unwrap()).into();

    let pref_cli = Client::new();

    Server::builder()
        .add_service(reflection)
        .add_service(user::new_server())
        .add_service(prefecture::new_server(pref_cli.clone()))
        .serve(addr)
        .await?;

    Ok(())
}
