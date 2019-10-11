extern crate sendgrid;

use sendgrid::v3::{
    Content, Email, Message as SendgridMessage, Personalization, Sender as SendgridSender,
};

use crate::provider::client::Sender;
use reqwest::StatusCode;

#[derive(Debug)]
pub struct SendgridClient {
    pub api_key: String,
}

impl Sender for SendgridClient {
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
        let p = Personalization::new().add_to(Email::new().set_email(&_to));
        let mut m = SendgridMessage::new()
            .set_from(Email::new().set_email(&_from))
            .set_subject(&_subject)
            .add_personalization(p);
        if _text.is_some() {
            m = m.add_content(
                Content::new()
                    .set_content_type("text/plain")
                    .set_value(&_text.unwrap()),
            )
        }
        if _html.is_some() {
            m = m.add_content(
                Content::new()
                    .set_content_type("text/html")
                    .set_value(&_html.unwrap()),
            )
        }
        let sender = SendgridSender::new(self.api_key.to_owned());
        match sender.send(&m) {
            Ok(result) => {
                if result.status() != StatusCode::OK {
                    match result.error_for_status() {
                        Err(text) => {
                            return Err(text.to_string());
                        }
                        Ok(_) => {
                            return Ok(());
                        }
                    }
                }
                return Ok(());
            }
            Err(error) => {
                return Err(error.to_string());
            }
        }
    }
}
