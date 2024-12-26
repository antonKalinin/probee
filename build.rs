use dotenv::dotenv;
use std::env;

fn main() {
    let dotenv_path = dotenv();

    // Use dotenv for local development
    if let Ok(dotenv_path) = dotenv_path {
        println!("cargo:rerun-if-changed={}", dotenv_path.display());
    }

    let api_key = env::var("ANTHROPIC_API_KEY").unwrap_or_default();
    let cmdi_api_url = env::var("CMDI_API_URL").unwrap_or_default();

    println!("cargo:rustc-env=ANTHROPIC_API_KEY={}", api_key);
    println!("cargo:rustc-env=CMDI_API_URL={}", cmdi_api_url);
}
