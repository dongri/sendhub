use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::app::{ResponseError, ResponseOk};
use crate::provider;
use crate::provider::client;
use crate::utils;
use crate::webhook;

#[derive(Deserialize, Default)]
pub struct PrimaryParams {
    primary: Option<String>,
    load_balancing: Option<bool>,
}

pub fn settings(params: web::Json<PrimaryParams>) -> HttpResponse {
    let primary = params.primary.to_owned();
    let load_balancing = params.load_balancing.to_owned();
    if primary.is_some() {
        let mut config = provider::SENDER_CONFIG.write().unwrap();
        let sender = provider::sender_from_str(primary.unwrap());
        config.sender_primary = sender;
    }
    if load_balancing.is_some() {
        let mut config = provider::SENDER_CONFIG.write().unwrap();
        config.load_balancing = load_balancing.unwrap();
    }
    HttpResponse::Ok().json(ResponseOk {
        result: String::from("ok"),
    })
}

#[derive(Deserialize, Default)]
pub struct RawParams {
    from: String,
    to: String,
    subject: String,
    html: Option<String>,
    text: Option<String>,
}

pub fn raw(params: web::Json<RawParams>) -> HttpResponse {
    let from = params.from.to_owned();
    let to = params.to.to_owned();
    let subject = params.subject.to_owned();
    let html = params.html.to_owned();
    let text = params.text.to_owned();
    if from == "" || to == "" || subject == "" {
        return HttpResponse::BadRequest().json(ResponseError {
            error: String::from("bad request"),
        });
    }
    let params = EmailParams {
        from: from,
        to: to,
        subject: subject,
        html: html,
        text: text,
    };
    match send_email_with_params(params) {
        Ok(_) => HttpResponse::Ok().json(ResponseOk {
            result: String::from("ok"),
        }),
        Err(err) => {
            println!("{:?}", err);
            HttpResponse::BadRequest().json(ResponseError { error: err })
        }
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct TemplateParams {
    from: String,
    to: String,
    subject: String,
    template: String,
    values: Value,
}

use serde_json::Value;

pub fn templae(params: web::Json<TemplateParams>) -> HttpResponse {
    let from = params.from.to_owned();
    let to = params.to.to_owned();
    let subject = params.subject.to_owned();
    let template_name = params.template.to_owned();
    let values = params.values.to_owned();

    if from == "" || to == "" || subject == "" {
        return HttpResponse::BadRequest().json(ResponseError {
            error: String::from("bad request"),
        });
    }
    let template_html = template_name.to_owned() + ".html";
    let template_text = template_name.to_owned() + ".text";
    let html: String;
    match utils::build_template(utils::string_to_static_str(template_html), &values) {
        Ok(build_html) => {
            html = build_html;
        }
        Err(err) => {
            println!("{:?}", err);
            return HttpResponse::BadRequest().json(ResponseError { error: err });
        }
    }
    let text: String;
    match utils::build_template(utils::string_to_static_str(template_text), &values) {
        Ok(build_text) => {
            text = build_text;
        }
        Err(err) => {
            println!("{:?}", err);
            return HttpResponse::BadRequest().json(ResponseError { error: err });
        }
    }

    let params = EmailParams {
        from: from,
        to: to,
        subject: subject,
        html: Some(html),
        text: Some(text),
    };
    match send_email_with_params(params) {
        Ok(_) => HttpResponse::Ok().json(ResponseOk {
            result: String::from("ok"),
        }),
        Err(err) => {
            println!("{:?}", err);
            HttpResponse::BadRequest().json(ResponseError { error: err })
        }
    }
}

struct EmailParams {
    from: String,
    to: String,
    subject: String,
    html: Option<String>,
    text: Option<String>,
}

fn send_email_with_params(params: EmailParams) -> Result<(), String> {
    let utc_datetime: DateTime<Utc> = Utc::now();
    let config = provider::SENDER_CONFIG.read().unwrap();
    let primary = &config.sender_primary;
    let load_balancing = config.load_balancing;
    if load_balancing == true {
        if utc_datetime.timestamp_millis() % 2 == 0 {
            send_email(client::ses(), params)
        } else {
            send_email(client::mailgun(), params)
        }
    } else {
        if primary == &provider::Sender::SES {
            send_email(client::ses(), params)
        } else if primary == &provider::Sender::Mailgun {
            send_email(client::mailgun(), params)
        } else if primary == &provider::Sender::Sendgrid {
            send_email(client::sendgrid(), params)
        } else {
            Err(String::from("provider error"))
        }
    }
}

fn send_email<T: client::Sender>(sender: T, params: EmailParams) -> Result<(), String> {
    let from = params.from;
    let to = params.to;
    let subject = params.subject;
    let html = params.html;
    let text = params.text;
    match sender.send(from, to, subject, html, text) {
        Ok(_) => Ok(()),
        Err(err) => {
            let text = format!("{}{}", "Sendhub Error: ", err);
            let slack_client = webhook::client::slack();
            let channel = "monitor-channel";
            match slack_client.send(channel, utils::string_to_static_str(text)) {
                Ok(_) => {}
                Err(err) => {
                    println!("slack error: {}", err);
                }
            }
            Err(err)
        }
    }
}
