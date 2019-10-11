use crate::env;

pub struct EnvLocal {}

impl env::config::Env for EnvLocal {
    fn name(&self) -> String {
        return env::config::ENV_LOCAL.to_string();
    }
    fn port(&self) -> String {
        return String::from("3000");
    }
    fn mailgun_domain(&self) -> String {
        return env::config::env_mailgun_domain();
    }
    fn mailgun_key(&self) -> String {
        return env::config::env_mailgun_key();
    }
    fn sendgrid_api_key(&self) -> String {
        return env::config::env_sendgrid_api_key();
    }
    fn aws_ses_key(&self) -> String {
        return env::config::env_aws_ses_key();
    }
    fn aws_ses_secret(&self) -> String {
        return env::config::env_aws_ses_secret();
    }
    fn slack_webhook_url(&self) -> String {
        return env::config::env_slack_webhook_url();
    }
}
