use nacos_sdk::api::config::{ConfigChangeListener, ConfigResponse, ConfigService};
use nacos_sdk::api::config::ConfigServiceBuilder;
use nacos_sdk::api::props::ClientProps;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub port: u16
}

pub static GLOBAL_CONFIG: Lazy<RwLock<AppConfig>> = Lazy::new(|| {
    RwLock::new(AppConfig {
        port: 8080
    })
});

static DATA_ID: &str = "Cassia.config.port";
static GROUP: &str = "DEFAULT_GROUP";

/// Initialize Nacos client and start config watching
pub async fn init_config_watcher() {
    let props = ClientProps::new()
        .server_addr("127.0.0.1:8848").auth_username("user").auth_password("user")
        .namespace("public");

    // ✅ 使用 builder 来创建 ConfigService
    let config_service = ConfigServiceBuilder::new(props)
        .build().unwrap();


    if let Ok(config) = config_service.get_config(DATA_ID.to_string(), GROUP.to_string()) {
        println!("[Nacos] Initial config pulled: {}", config);
        let content: &str = config.content().as_str();
        apply_config(content);
    }

    let result = config_service.add_listener(
        DATA_ID.to_string(),
        GROUP.to_string(),
        Arc::new(NacosConfigListener),
    );

    if let Err(err) = result {
        eprintln!("Add listener failed: {:?}", err);
    }

    println!("[Nacos] Config watcher started");
}

struct NacosConfigListener;

impl ConfigChangeListener for NacosConfigListener {
    fn notify(&self, config_resp: ConfigResponse) {
        println!("[Nacos] Config changed: {:?}", config_resp.content());
        apply_config(&config_resp.content());
    }
}

fn apply_config(json_str: &str) {
    match serde_json::from_str::<AppConfig>(json_str) {
        Ok(cfg) => {
            let mut global = GLOBAL_CONFIG.write().unwrap();
            *global = cfg.clone();
            println!("[Nacos] New config applied: {:?}", cfg);
        }
        Err(e) => {
            eprintln!("[Nacos] Failed to parse config JSON: {}", e);
        }
    }
}
