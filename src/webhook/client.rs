use crate::env;
use crate::webhook::slack::SlackClient;

pub fn slack() -> SlackClient {
    SlackClient {
        url: &env::ENV_CONFIG.slack_webhook_url,
    }
}
