use tonic::{Request, Response, Status};
use crate::module::users::proto::user_service_server::{UserService};
use crate::module::users::proto::user_proto::{CreateUserRequest,GetUserRequest,UserResponse,UserRole};
use crate::module::users::proto::user_proto::{Address};
#[derive(Default)]
pub struct MyUserService {}

#[tonic::async_trait]
impl UserService for MyUserService {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();

        let reply = UserResponse {
            user_id: 1,
            name: req.name,
            email: req.email,
            role: req.role,
            tags: req.tags,
            address: req.address,
            metadata: req.metadata,
        };

        Ok(Response::new(reply))
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {

        let req = request.into_inner();
        let reply = UserResponse {
            user_id: req.user_id,
            name: format!("User{}", req.user_id),
            email: format!("user{}@example.com", req.user_id),
            role: UserRole::Admin as i32,
            tags: vec!["mock".into(), "test".into()],
            address: Some(Address {
                country: "MockCountry".into(),
                province: "MockProvince".into(),
                city: "MockCity".into(),
                street: "123 Mock Street".into(),
            }),
            metadata: Default::default(),
        };

        Ok(Response::new(reply))
    }
}
