use std::env as stdenv;

pub mod config;
pub mod development;
pub mod docker;
pub mod local;
pub mod production;

lazy_static! {
    pub static ref ENV_CONFIG: config::Config = {
        let rust_env = match stdenv::var("RUST_ENV") {
            Ok(val) => val,
            Err(err) => {
                println!("{}: {}", err, "RUST_ENV");
                String::from("local")
            }
        };
        let config = config::get_env_config(rust_env);
        return config;
    };
}
