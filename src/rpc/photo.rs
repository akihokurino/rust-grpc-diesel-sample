use tonic::{Request, Response, Status};

use pb::photo_service_server::{PhotoService, PhotoServiceServer};

pub mod common {
    tonic::include_proto!("common");
}
pub mod pb {
    tonic::include_proto!("photo");
}

pub fn new_server() -> PhotoServiceServer<PhotoServiceImpl> {
    PhotoServiceServer::new(PhotoServiceImpl::default())
}

#[derive(Debug, Default)]
pub struct PhotoServiceImpl {}

#[tonic::async_trait]
impl PhotoService for PhotoServiceImpl {
    async fn list(
        &self,
        request: Request<common::Empty>,
    ) -> Result<Response<pb::PhotoList>, Status> {
        println!("Got a request: {:?}", request);

        let reply = pb::Photo {
            id: "1".to_string(),
            url: "https://placehold.jp/150x150.png".to_string(),
        };
        Ok(Response::new(pb::PhotoList { items: vec![reply] }))
    }
}
