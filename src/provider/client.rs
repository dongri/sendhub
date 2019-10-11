use crate::env;
use crate::provider::mailgun::MailgunClient;
use crate::provider::sendgrid::SendgridClient;
use crate::provider::ses::SESClient;

pub trait Sender {
    fn send(
        &self,
        from: impl Into<String>,
        to: impl Into<String>,
        subject: impl Into<String>,
        html: impl Into<Option<String>>,
        text: impl Into<Option<String>>,
    ) -> Result<(), String>;
}

pub fn ses() -> SESClient {
    SESClient {
        access_key: env::ENV_CONFIG.aws_ses_key.to_owned(),
        secret_access_key: env::ENV_CONFIG.aws_ses_secret.to_owned(),
    }
}

pub fn mailgun() -> MailgunClient {
    MailgunClient {
        domain: env::ENV_CONFIG.mailgun_domain.to_owned(),
        key: env::ENV_CONFIG.mailgun_key.to_owned(),
    }
}

pub fn sendgrid() -> SendgridClient {
    SendgridClient {
        api_key: env::ENV_CONFIG.sendgrid_api_key.to_owned(),
    }
}
