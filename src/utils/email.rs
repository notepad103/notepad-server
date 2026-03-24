use crate::AppError;
use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use std::sync::LazyLock;

// const SMTP_SERVER: &str = "smtp.qq.com";
// const SMTP_PORT: u16 = 587;
// const AUTH_CODE: &str = "123456";

pub struct to_email {
    pub to_email: String,
    pub subject: String,
    pub body: String,
}

struct smtp_transport_builder<'a> {
    pub from_email: &'a str,
    pub smtp_server: &'a str,
    pub smtp_port: u16,
    pub auth_code: &'a str,
}

pub struct smtp_transport;

const SMTP_TRANSPORT_CONFIG: smtp_transport_builder<'static> = smtp_transport_builder {
    from_email: "123456@qq.com",
    smtp_server: "imap.qq.com",
    smtp_port: 465,
    auth_code: "knczektozzvrbaai",
};

static SMTP_TRANSPORT: LazyLock<SmtpTransport> = LazyLock::new(|| {
    SmtpTransport::builder_dangerous(SMTP_TRANSPORT_CONFIG.smtp_server)
        .port(SMTP_TRANSPORT_CONFIG.smtp_port)
        .credentials(Credentials::new(
            SMTP_TRANSPORT_CONFIG.from_email.to_string(),
            SMTP_TRANSPORT_CONFIG.auth_code.to_string(),
        ))
        .build()
});

impl smtp_transport {
    fn transport() -> &'static SmtpTransport {
        &SMTP_TRANSPORT
    }

    pub async fn send_email(data: to_email) -> Result<(), AppError> {
        let email = Message::builder()
            .from(
                SMTP_TRANSPORT_CONFIG
                    .from_email
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
            subject: "Verification Code".to_string(),
            body: format!("Your verification code is: {code}"),
        };
        Self::send_email(data).await
    }
}
