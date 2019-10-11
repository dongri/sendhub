use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::app::{ResponseError, ResponseOk};
use crate::utils;
use crate::webhook;

#[derive(Deserialize, Default)]
pub struct SlackParams {
    channel: String,
    text: String,
}

pub fn slack_message(params: web::Json<SlackParams>) -> impl Responder {
    let channel = params.channel.to_owned();
    let text = params.text.to_owned();

    let slack_client = webhook::client::slack();
    let res = slack_client.send(
        utils::string_to_static_str(channel),
        utils::string_to_static_str(text),
    );
    match res {
        Ok(_) => HttpResponse::Ok().json(ResponseOk {
            result: String::from("ok"),
        }),
        Err(err) => {
            println!("{:?}", err);
            HttpResponse::BadRequest().json(ResponseError { error: err })
        }
    }
}
