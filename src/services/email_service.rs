use std::error::Error;

use lettre::message::Mailbox;
use lettre::AsyncTransport;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    Message, Tokio1Executor,
};
use log::error;

use crate::api::error_response::ErrorResponse;
use crate::app_state::EnvironmentVariables;
use crate::constants::{ENV_EMAIL_PASSWORD, ENV_EMAIL_RELAY, ENV_EMAIL_USERNAME};
use crate::util::error_to_response;

pub struct EmailService {
    email: Mailbox,
    mailer: AsyncSmtpTransport<Tokio1Executor>,
}
impl EmailService {
    pub fn new(env: &EnvironmentVariables) -> Self {
        let credentials = Credentials::new(
            env.get(ENV_EMAIL_USERNAME).to_string(),
            env.get(ENV_EMAIL_PASSWORD).to_string(),
        );
        Self {
            email: env
                .get(ENV_EMAIL_USERNAME)
                .parse()
                .expect("Invalid sender email"),
            mailer: AsyncSmtpTransport::<Tokio1Executor>::relay(env.get(ENV_EMAIL_RELAY))
                .expect("Invalid relay")
                .credentials(credentials)
                .build(),
        }
    }

    async fn try_send_text_mail(
        &self,
        to: String,
        subject: String,
        body: String,
    ) -> Result<(), Box<dyn Error>> {
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
            }
        }
    }

    pub async fn send_text_mail(
        &self,
        to: String,
        subject: String,
        body: String,
    ) -> Result<(), ErrorResponse> {
        if let Err(err) = self.try_send_text_mail(to, subject, body).await {
            Err(error_to_response(err))
        } else {
            Ok(())
        }
    }
}
