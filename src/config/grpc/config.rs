use tonic::server::NamedService;
use tonic::transport::{Server, server::Router};
use crate::module::users::services::MyUserService;
use crate::module::user_service_server::UserServiceServer;

pub type GrpcRegisterFn<L> = fn(Router<L>) -> Router<L>;
pub struct GrpcServiceConfig<L> {
    pub name: &'static str,
    pub port: u16,
    pub methods: Vec<String>,
    // 返回 dyn NamedService
    pub register: GrpcRegisterFn<L>,
}

/// Build list of gRPC service configs to be registered and served.
pub fn build_grpc_config<L>() -> Vec<GrpcServiceConfig<L>> {
    vec![
        GrpcServiceConfig {
            name: "user-service",
            methods: vec!["getUser".to_string(), "postUser".to_string()],
            port: 50051,
            register: |router| {
                router.add_service(UserServiceServer::new(MyUserService::default()))
            },
        },
    ]
}
