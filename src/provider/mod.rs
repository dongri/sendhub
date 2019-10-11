use std::sync::RwLock;

pub mod client;
pub mod mailgun;
pub mod sendgrid;
pub mod ses;

#[derive(PartialEq, Debug)]
pub enum Sender {
    SES,
    Mailgun,
    Sendgrid,
}

pub fn sender_from_str(name: String) -> Sender {
    match name.as_str() {
        "ses" => Sender::SES,
        "mailgun" => Sender::Mailgun,
        "sendgrid" => Sender::Sendgrid,
        _ => Sender::SES,
    }
}

#[derive(Debug)]
pub struct SenderCofig {
    pub sender_primary: Sender,
    pub sender_secondary: Sender,
    pub sender_standby: Sender,
    pub load_balancing: bool,
}

lazy_static! {
    #[derive(Debug)]
    pub static ref SENDER_CONFIG: RwLock<SenderCofig> = {
        let c = SenderCofig {
            sender_primary: Sender::SES,
            sender_secondary: Sender::Mailgun,
            sender_standby: Sender::Sendgrid,
            load_balancing: false,
        };
        RwLock::new(c)
    };
}
