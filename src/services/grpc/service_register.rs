use std::net::SocketAddr;
use std::collections::HashMap;
use nacos_sdk::api::naming::{NamingService, NamingServiceBuilder, ServiceInstance};
use nacos_sdk::api::props::ClientProps;
use crate::config::grpc::{build_grpc_config, GrpcServiceConfig};
use tonic::transport::Server;
use crate::services::grpc::empty_imp::MyEmptyService;
use crate::services::grpc::empty_service_server::{EmptyServiceServer};
const SERVICE_NAME: &str = "all-in-one-grpc";
const GROUP_NAME: &str = "DEFAULT_GROUP";

fn build_service_instance<L>(config : &GrpcServiceConfig<L>,ip: &str, port: i32) -> ServiceInstance {

    let mut metadata: HashMap<String, String> = HashMap::new();
    metadata.insert("protocol".into(), "grpc".into());
    let method_list = config.methods.join(",");
    metadata.insert("methods".into(), method_list);

    ServiceInstance {
        service_name: Some(SERVICE_NAME.to_string()),
        ip: ip.to_string(),
        port,
        cluster_name: Some("DEFAULT".into()),
        healthy: true,
        ephemeral: true,
        metadata: metadata,
        ..Default::default()
    }

}
fn register_to_nacos<L>() -> Result<(), Box<dyn std::error::Error>> {
    let ip = local_ip_address::local_ip()?.to_string();
    let port = 50051;
    let addr: SocketAddr = format!("{}:{}", ip, port).parse()?;
    let props = ClientProps::new()
        .server_addr("127.0.0.1:8848")
        .namespace("public");

    let naming_service = NamingServiceBuilder::new(props).build()?;
    let configs = build_grpc_config::<L>();
    for cfg in configs {
        let instance = build_service_instance(&cfg,&ip, port);
        naming_service.register_instance(
            SERVICE_NAME.to_string(),
            Some(GROUP_NAME.to_string()),
            instance,
        )?;
    }

    println!("[Nacos] Service registered to Nacos successfully");
    Ok(())
}

pub async fn start_grpc_server() -> Result<(), Box<dyn std::error::Error>> {
    let ip = local_ip_address::local_ip()?.to_string();
    let port = 50051;
    let addr: SocketAddr = format!("{}:{}", ip, port).parse()?;
    register_to_nacos::<MyEmptyService>()?;
    let config_list = build_grpc_config();
    let mut router = Server::builder().add_service(EmptyServiceServer::new(MyEmptyService::default()));
    for config in config_list {
        router = (config.register)(router);
    }

    router.serve(addr).await?;
    println!("[GRPC] Server started at {}", addr);
    Ok(())
}

pub async fn refresh_grpc_server() -> Result<(), Box<dyn std::error::Error>> {
    println!("[GRPC] Refreshing gRPC server...");
     start_grpc_server().await
}

