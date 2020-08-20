use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap_or("./static".to_string());

    println!("Building web in {:?}", out_dir);
    // cmd: wasm-pack build --target web --out-name wasm --out-dir ./static
    Command::new("wasm-pack")
        .arg(&format!("--target web"))
        .arg(&format!("--out-name wasm"))
        .arg(&format!("--out-dir {}", out_dir))
        .status()
        .unwrap();
}
