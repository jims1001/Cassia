
use tonic::{Request, Response, Status};
use crate::services::grpc::Empty;
use crate::services::grpc::empty_service_server::EmptyService;

pub mod empty {

}

#[derive(Default)]
pub struct MyEmptyService;

#[tonic::async_trait]
impl EmptyService for MyEmptyService {
    async fn ping(&self, _req: Request<Empty>) -> Result<Response<Empty>, Status> {
        Ok(Response::new(Empty {}))
    }
}
