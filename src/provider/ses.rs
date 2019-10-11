extern crate rusoto_core;
extern crate rusoto_ses;

use mimers::{Encoding, WordEncoder};
use rusoto_core::{HttpClient, Region};
use rusoto_credential::StaticProvider;
use rusoto_ses::{
    Body, Content as SESContent, Destination, Message as SESMessage, SendEmailRequest, Ses,
    SesClient,
};

use crate::provider::client::Sender;
use std::error::Error;

#[derive(Debug)]
pub struct SESClient {
    pub access_key: String,
    pub secret_access_key: String,
}

impl Sender for SESClient {
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
        let static_provider = StaticProvider::new_minimal(
            self.access_key.to_owned(),
            self.secret_access_key.to_owned(),
        );
        let client =
            SesClient::new_with(HttpClient::new().unwrap(), static_provider, Region::UsEast1);
        let v: Vec<&str> = _from.split("<").collect();
        let mut source = _from.to_owned();
        if v.len() == 2 {
            let email_name = v[0];
            let email_address = v[1];
            let encoder = WordEncoder::new("UTF-8", Encoding::QEncoding);
            let encoded_email_name = encoder.encode_word(email_name.to_owned());
            source = format!("{}<{}", encoded_email_name, email_address);
        }
        let mut html_string = _html;
        let text_string = _text;
        if html_string.is_none() && text_string.is_some() {
            html_string = Some(text_string.to_owned().unwrap().replace("\n", "<br>"));
        }
        let destination = Destination {
            to_addresses: Some(vec![_to]),
            ..Default::default()
        };
        let html_content = SESContent {
            data: html_string.unwrap_or("".to_string()),
            ..Default::default()
        };
        let text_content = SESContent {
            data: text_string.unwrap_or("".to_string()),
            ..Default::default()
        };
        let subject_content = SESContent {
            data: _subject,
            ..Default::default()
        };
        let body = Body {
            html: Some(html_content),
            text: Some(text_content),
            ..Default::default()
        };
        let message = SESMessage {
            body: body,
            subject: subject_content,
            ..Default::default()
        };
        let send_email_request = SendEmailRequest {
            destination: destination,
            message: message,
            source: source,
            ..Default::default()
        };
        match client.send_email(send_email_request).sync() {
            Ok(_) => {
                return Ok(())
            }
            Err(err) => {
                return Err(err.description().to_string())
            }
        };
    }
}
