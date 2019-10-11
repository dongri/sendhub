use std::env as stdenv;

use crate::env::development::EnvDevelopment;
use crate::env::docker::EnvDocker;
use crate::env::local::EnvLocal;
use crate::env::production::EnvProduction;

pub const ENV_LOCAL: &str = "local";
pub const ENV_DOCKER: &str = "docker";
pub const ENV_DEVELOPMENT: &str = "development";
pub const ENV_PRODUCTION: &str = "production";

pub struct Config {
    pub name: String,
    pub port: String,
    pub mailgun_domain: String,
    pub mailgun_key: String,
    pub sendgrid_api_key: String,
    pub aws_ses_key: String,
    pub aws_ses_secret: String,
    pub slack_webhook_url: String,
}

pub trait Env {
    fn name(&self) -> String;
    fn port(&self) -> String;
    fn mailgun_domain(&self) -> String;
    fn mailgun_key(&self) -> String;
    fn sendgrid_api_key(&self) -> String;
    fn aws_ses_key(&self) -> String;
    fn aws_ses_secret(&self) -> String;
    fn slack_webhook_url(&self) -> String;
}

pub fn get_env_config(rust_env: String) -> Config {
    match rust_env.as_str() {
        ENV_DOCKER => build_config(EnvDocker {}),
        ENV_LOCAL => build_config(EnvLocal {}),
        ENV_DEVELOPMENT => build_config(EnvDevelopment {}),
        ENV_PRODUCTION => build_config(EnvProduction {}),
        _ => build_config(EnvLocal {}),
    }
}

fn build_config<T: Env>(v: T) -> Config {
    let config = Config {
        name: v.name(),
        port: v.port(),
        mailgun_domain: v.mailgun_domain(),
        mailgun_key: v.mailgun_key(),
        sendgrid_api_key: v.sendgrid_api_key(),
        aws_ses_key: v.aws_ses_key(),
        aws_ses_secret: v.aws_ses_secret(),
        slack_webhook_url: v.slack_webhook_url(),
    };
    return config;
}

pub fn env_mailgun_domain() -> String {
    return stdenv::var("MAILGUN_DOMAIN").expect("maingun_domain is empty");
}
pub fn env_mailgun_key() -> String {
    return stdenv::var("MAILGUN_KEY").expect("mailgun_key is empty");
}
pub fn env_sendgrid_api_key() -> String {
    return stdenv::var("SENDGRID_API_KEY").expect("sendgrid_api_key is empty");
}
pub fn env_aws_ses_key() -> String {
    return stdenv::var("AWS_SES_KEY").expect("aws_ses_key is empty");
}
pub fn env_aws_ses_secret() -> String {
    return stdenv::var("AWS_SES_SECRET").expect("aws_ses_secret is empty");
}
pub fn env_slack_webhook_url() -> String {
    return stdenv::var("SLACK_WEBHOOK_URL").expect("slack_webhook_url is empty");
}
