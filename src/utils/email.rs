use crate::AppError;
use crate::Config;
use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use std::sync::LazyLock;

pub struct to_email {
    pub to_email: String,
    pub subject: String,
    pub body: String,
}

pub struct smtp_transport;

static SMTP_TRANSPORT: LazyLock<SmtpTransport> = LazyLock::new(|| {
    let c = Config::global();
    // `relay`：默认 SMTPS（465 + TLS wrapper）；host 须为 SMTP，勿填 `imap.*`
    SmtpTransport::relay(c.smtp_host.as_str())
        .expect("smtp relay (tls) init failed; check SMTP_HOST and lettre TLS features")
        .port(c.smtp_port)
        .credentials(Credentials::new(
            c.smtp_from_email.clone(),
            c.smtp_auth_code.clone(),
        ))
        .build()
});

impl smtp_transport {
    fn transport() -> &'static SmtpTransport {
        &SMTP_TRANSPORT
    }

    pub async fn send_email(data: to_email) -> Result<(), AppError> {
        let from = Config::global().smtp_from_email.as_str();
        let email = Message::builder()
            .from(
                from
                    .parse()
                    .map_err(|e| AppError::BadRequest(format!("invalid from email: {e}")))?,
            )
            .to(data
                .to_email
                .parse()
                .map_err(|e| AppError::BadRequest(format!("invalid to email: {e}")))?)
            .subject(data.subject.to_string())
            .body(data.body.to_string())
            .map_err(|e| AppError::BadRequest(format!("failed to build email: {e}")))?;
        Self::transport()
            .send(&email)
            .map_err(|e| AppError::Internal(format!("failed to send email: {e}")))?;

        Ok(())
    }
    pub async fn send_verification_code(email: &str, code: u32) -> Result<(), AppError> {
        let data = to_email {
            to_email: email.to_string(),
            subject: "验证码".to_string(),
            body: format!("您的验证码是：{code}"),
        };
        Self::send_email(data).await
    }
}
