use axum::async_trait;
use resend_rs::{types::CreateEmailBaseOptions, Error, Resend};

#[derive(Clone)]
pub struct Mailer {
    pub client: Resend,
}

#[async_trait]
impl Mail for Mailer {
    async fn mail(&self, email: CreateEmailBaseOptions) -> Result<(), resend_rs::Error> {
        match self.client.emails.send(email).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

#[async_trait]
pub trait Mail: MailClone + Send + Sync {
    async fn mail(&self, email: CreateEmailBaseOptions) -> Result<(), Error>;
}

pub trait MailClone {
    fn clone_box(&self) -> Box<dyn Mail>;
}

impl<T> MailClone for T
where
    T: 'static + Mail + Clone,
{
    fn clone_box(&self) -> Box<dyn Mail> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Mail> {
    fn clone(&self) -> Box<dyn Mail> {
        self.clone_box()
    }
}
