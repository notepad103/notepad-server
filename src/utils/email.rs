use lettre::{SmtpTransport, transport::smtp::authentication::Credentials};

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

pub struct smtp_transport {
    pub smtp_transport: SmtpTransport,
}

const SMTP_TRANSPORT_CONFIG: smtp_transport_builder<'static> = smtp_transport_builder {
    from_email: "123456@qq.com",
    smtp_server: "imap.qq.com",
    smtp_port: 465,
    auth_code: "knczektozzvrbaai",
};

impl smtp_transport {
    pub fn new() -> Self {
        let smtp_transport = SmtpTransport::builder_dangerous(SMTP_TRANSPORT_CONFIG.smtp_server)
            .port(SMTP_TRANSPORT_CONFIG.smtp_port)
            .credentials(Credentials::new(
                SMTP_TRANSPORT_CONFIG.from_email.to_string(),
                SMTP_TRANSPORT_CONFIG.auth_code.to_string(),
            ))
            .build();
        Self { smtp_transport }
    }
}
