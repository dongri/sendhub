extern crate slack_hook;

use slack_hook::{PayloadBuilder, Slack};

#[derive(Debug)]
pub struct SlackClient {
    pub url: &'static str,
}

impl SlackClient {
    pub fn send(&self, channel: &'static str, text: &'static str) -> Result<(), String> {
        let slack = Slack::new(self.url).unwrap();
        let p = PayloadBuilder::new()
            .text(text)
            .channel(channel)
            .username("bot")
            .icon_emoji(":shit:")
            .link_names(true)
            .build()
            .unwrap();

        let res = slack.send(&p);
        match res {
            Ok(_) => Ok(()),
            Err(x) => Err(x.to_string()),
        }
    }
}
