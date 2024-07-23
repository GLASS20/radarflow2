use std::error::Error;

use serde::{Deserialize, Serialize};
use vergen::EmitBuilder;

#[derive(Clone, Deserialize, Serialize)]
struct InfoJson {
    build_number: usize,
    timestamp: String
}

fn download(url: &str, to: &str) -> Result<(), Box<dyn Error>> {
    let content = reqwest::blocking::get(url)
        .unwrap_or_else(|_| panic!("Downloading \"{to}\""))
        .text()
        .expect("Convert response to text");

    std::fs::write(to, content)
    .expect("Write to file");

    Ok(())
}

fn build_number() -> Result<(), Box<dyn Error>> {
    let content = reqwest::blocking::get("https://mirror.ghproxy.com/https://raw.githubusercontent.com/a2x/cs2-dumper/main/output/info.json")
        .unwrap_or_else(|_| panic!("Downloading info.json"))
        .text()
        .expect("Convert response to text");

    let info = serde_json::from_str::<InfoJson>(&content)?;
    println!("cargo:rustc-env=CS2_BUILD_NUMBER={}", info.build_number);
	@@ -34,21 +36,20 @@ fn build_number() -> Result<(), Box<dyn Error>> {
}

fn main() -> Result<(), Box<dyn Error>> {

    download(
        "https://mirror.ghproxy.com/https://raw.githubusercontent.com/a2x/cs2-dumper/main/output/client.dll.rs",
        "./src/dma/cs2dumper/client_mod.rs"
    ).expect("Failed to download build file \"client.dll.rs\"");

    download(
        "https://mirror.ghproxy.com/https://raw.githubusercontent.com/a2x/cs2-dumper/main/output/offsets.rs",
        "./src/dma/cs2dumper/offsets_mod.rs"
    ).expect("Failed to download build file \"offsets.rs\"");

    download(
        "https://mirror.ghproxy.com/https://raw.githubusercontent.com/a2x/cs2-dumper/main/output/engine2.dll.rs",
        "./src/dma/cs2dumper/engine2_mod.rs"
    ).expect("Failed to download build file \"engine2.dll.rs\"");

    build_number()?;

	@@ -62,4 +63,4 @@ fn main() -> Result<(), Box<dyn Error>> {
        .emit()?;

    Ok(())
}
