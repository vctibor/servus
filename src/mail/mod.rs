/*
use lettre::smtp::authentication::Credentials;
use lettre::{EmailAddress, Envelope, SendableEmail, SmtpClient, Transport};

pub fn send_mail() {
    let email = SendableEmail::new(
        Envelope::new(
            Some(EmailAddress::new("vladimir.malky@gmail.com".to_string()).unwrap()),
            vec![EmailAddress::new("vladimir.malky@gmail.com".to_string()).unwrap()],
        )
            .unwrap(),
        "id".to_string(),
        "Hello example".to_string().into_bytes(),
    );

    let creds = Credentials::new(
        "vladimir.malky@gmail.com".to_string(),
        "hunter2".to_string(),
    );

    // Open a remote connection to gmail
    let mut mailer = SmtpClient::new_simple("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .transport();

    // Send the email
    let result = mailer.send(email);

    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Could not send email: {:?}", result);
    }
}
*/