use dotenv::dotenv;
use std::env;

fn main() {
    let dotenv_path = dotenv();

    // Use dotenv for local development
    if let Ok(dotenv_path) = dotenv_path {
        println!("cargo:rerun-if-changed={}", dotenv_path.display());
    }

    let api_key = env::var("ANTHROPIC_API_KEY").unwrap_or_default();
    let storage_salt = env::var("STORAGE_SALT").unwrap_or_default();
    let supabase_public_url = env::var("SUPABASE_PUBLIC_URL").unwrap_or_default();
    let supabase_public_anon_key = env::var("SUPABASE_PUBLIC_ANON_KEY").unwrap_or_default();

    println!("cargo:rustc-env=ANTHROPIC_API_KEY={}", api_key);
    println!("cargo:rustc-env=STORAGE_SALT={}", storage_salt);
    println!(
        "cargo:rustc-env=SUPABASE_PUBLIC_URL={}",
        supabase_public_url
    );
    println!(
        "cargo:rustc-env=SUPABASE_PUBLIC_ANON_KEY={}",
        supabase_public_anon_key
    );
}
