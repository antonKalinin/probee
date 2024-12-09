use std::env;

fn main() {
    let api_key =
        env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY environment variable not set");

    // Pass the environment variable to Cargo (to be accessed in code)
    println!("cargo:rerun-if-env-changed=ANTHROPIC_API_KEY");
    println!("cargo:rerun-if-changed=build.rs");

    // Make the environment variable available to the Rust code
    println!("cargo:rustc-env=ANTHROPIC_API_KEY={}", api_key);
}
