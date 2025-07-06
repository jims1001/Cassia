use std::process::Command;
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:warning=👉 start check librdkafka.a is exist...");

    let static_lib_path = Path::new("build/librdkafka/src-cpp/librdkafka.a");

    if !static_lib_path.exists() {
        println!("cargo:warning=⚠️ not found .a file，building...");
        Command::new("sh")
            .arg("scripts/link.sh")
            .status()
            .expect("Failed to execute shell script");


    } else {
        println!("✅  librdkafka.a，skip compile");
    }

    let current_dir: PathBuf = env::current_dir().expect("get current dir is failed");
    let lib_path = current_dir.join("third_party/librdkafka/src-cpp");
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=dylib=rdkafka");
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/local/include/librdkafka");




}