extern crate mailgun_rs;

use mailgun_rs::{EmailAddress, Mailgun, Message};

use crate::provider::client::Sender;
use std::error::Error;

#[derive(Debug)]
pub struct MailgunClient {
    pub domain: String,
    pub key: String,
}

impl Sender for MailgunClient {
    fn send(
        &self,
        from: impl Into<String>,
        to: impl Into<String>,
        subject: impl Into<String>,
        html: impl Into<Option<String>>,
        text: impl Into<Option<String>>,
    ) -> Result<(), String> {
        let _from = from.into();
        let _to = to.into();
        let _subject = subject.into();
        let _html = html.into();
        let _text = text.into();
        let domain = self.domain.to_owned();
        let key = self.key.to_owned();
        let recipient = EmailAddress::address(&_to);
        let mut message = Message {
            to: vec![recipient],
            subject: _subject,
            ..Default::default()
        };
        if _html.is_some() {
            message.html = _html.unwrap();
        }
        if _text.is_some() {
            message.text = _text.unwrap();
        }
        let client = Mailgun {
            api_key: key,
            domain: domain,
            message: message,
        };
        let sender = EmailAddress::address(&_from);

        match client.send(&sender) {
            Ok(_) => {
                return Ok(());
            }
            Err(error) => {
                return Err(error.description().to_string());
            }
        }
    }
}
