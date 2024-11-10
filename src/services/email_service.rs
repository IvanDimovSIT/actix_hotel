use std::error::Error;

use lettre::message::Mailbox;
use lettre::{message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport, Message, Tokio1Executor};
use lettre::AsyncTransport;
use log::error;

use crate::app_state::EnvironmentVariables;
use crate::constants::{ENV_EMAIL_PASSWORD, ENV_EMAIL_RELAY, ENV_EMAIL_USERNAME};

pub struct MailService{
    email: Mailbox,
    mailer: AsyncSmtpTransport<Tokio1Executor>,
}
impl MailService {
    pub fn new(env: &EnvironmentVariables) -> Self {
        let credentials = Credentials::new(
            env.get(ENV_EMAIL_USERNAME).to_string(), 
            env.get(ENV_EMAIL_PASSWORD).to_string()
        );
        Self {
            email: env.get(ENV_EMAIL_USERNAME)
                .parse()
                .expect("Invalid sender email"), 
            mailer: AsyncSmtpTransport::<Tokio1Executor>::relay(env.get(ENV_EMAIL_RELAY))
            .unwrap()
            .credentials(credentials)
            .build()
        }
    }

    pub async fn send_text_mail(&self, to: String, subject: String, body: String) -> Result<(), Box<dyn Error>> {
        let email = Message::builder()
            .from(self.email.clone())
            .to(to.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)?;

        match self.mailer.send(email).await {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("Could not send email: {err}");
                Err(Box::new(err))
            },
        }
    }
}
