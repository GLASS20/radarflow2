use std::error::Error;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use vergen::EmitBuilder;
use reqwest::blocking::Client;

#[derive(Clone, Deserialize, Serialize)]
struct InfoJson {
    build_number: usize,
    timestamp: String,
}

fn download(url: &str, to: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10)) // 设置请求超时时间为10秒
        .build()?;

    let content = client.get(url).send()?.text()?;

    std::fs::write(to, content)?;

    Ok(())
}

fn build_number() -> Result<(), Box<dyn Error>> {
    let content = Client::new()
        .get("https://raw.githubusercontent.com/a2x/cs2-dumper/main/output/info.json")
        .send()?
        .text()?;

    let info = serde_json::from_str::<InfoJson>(&content)?;
    println!("cargo:rustc-env=CS2_BUILD_NUMBER={}", info.build_number);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    download(
        "https://raw.githubusercontent.com/a2x/cs2-dumper/main/output/client.dll.rs",
        "./src/dma/cs2dumper/client_mod.rs",
    )?;

    download(
        "https://raw.githubusercontent.com/a2x/cs2-dumper/main/output/offsets.rs",
        "./src/dma/cs2dumper/offsets_mod.rs",
    )?;

    download(
        "https://raw.githubusercontent.com/a2x/cs2-dumper/main/output/engine2.dll.rs",
        "./src/dma/cs2dumper/engine2_mod.rs",
    )?;

    build_number()?;

    EmitBuilder::builder()
        .git_sha(true)
        .git_commit_date()
        .cargo_debug()
        .cargo_target_triple()
        .rustc_semver()
        .rustc_llvm_version()
        .emit()?;

    Ok(())
}
