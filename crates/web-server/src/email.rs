use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn send_email(config: &super::config::Config, email: Message) {
    if let Some(smtp_config) = &config.smtp_config {
        let creds = Credentials::new(smtp_config.username.clone(), smtp_config.password.clone());

        let sender = if smtp_config.tls_off {
            SmtpTransport::builder_dangerous(smtp_config.host.clone())
                .port(smtp_config.port)
                .credentials(creds)
                .build()
        } else {
            SmtpTransport::relay(&smtp_config.host)
                .unwrap()
                .port(smtp_config.port)
                .credentials(creds)
                .build()
        };

        // Send the email
        match sender.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }
}
