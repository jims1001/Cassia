use std::net::SocketAddr;
use actix::fut::ok;
use tonic::transport::Server;

use nacos_sdk::api::naming::{NamingService, NamingServiceBuilder, ServiceInstance};
use nacos_sdk::api::props::ClientProps;
use local_ip_address;
use tokio::signal;
use crate::module::user_service_server::UserServiceServer;
use crate::module::users::services::MyUserService;
pub async fn start_grpc_server() -> Result<(), Box<dyn std::error::Error>> {
    // let ip = local_ip_address::local_ip()?.to_string();
    // let port = 50051;
    // let addr: SocketAddr = format!("{}:{}", ip, port).parse()?;
    //
    // let user_service = MyUserService::default();
    //
    // let props = ClientProps::new()
    //     .server_addr("127.0.0.1:8848")
    //     .namespace("public");
    //
    // let naming_service = NamingServiceBuilder::new(props)
    //     .build().unwrap()
    //
    // let instance = ServiceInstance {
    //     service_name: "user-service".to_string(),
    //     ip: ip.clone(),
    //     port,
    //     cluster_name: Some("DEFAULT".to_string()),
    //     weight: Some(1.0),
    //     healthy: Some(true),
    //     ephemeral: Some(true),
    //     metadata: Some(
    //         [("protocol".to_string(), "grpc".to_string())]
    //             .iter()
    //             .cloned()
    //             .collect(),
    //     ),
    //     ..Default::default()
    // };
    //
    // // 注册服务
    // naming_service
    //     .register_instance_with_instance(instance.clone())
    //     .await
    //     .expect("Failed to register gRPC instance");
    //
    // println!("✅ gRPC server listening on {}", addr);
    //
    // // 用 tokio::select! 监听 gRPC 和 Ctrl+C
    // tokio::select! {
    //     result = Server::builder()
    //         .add_service(UserServiceServer::new(user_service))
    //         .serve(addr) => {
    //             result?;
    //         }
    //
    //     _ = signal::ctrl_c() => {
    //         println!("🛑 Received Ctrl+C, shutting down...");
    //     }
    // }
    //
    // // 注销服务
    // naming_service
    //     .deregister_instance_with_instance(instance.clone())
    //     .await
    //     .expect("Failed to deregister instance");
    //
    // println!("🧹 Deregistered from Nacos, exiting.");
    // Ok(())

    Ok(())

}
