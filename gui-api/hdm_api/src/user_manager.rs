use pam_client::{Context, Flag};
use pam_client::conv_mock::Conversation;
use otpauth::{TOTP, HOTP};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub enum TwoFactorMethod {
    TOTP,
    HOTP { counter: u64 },
    None,
}

pub struct User {
    pub username: String,
    pub pam_service: String,
    pub twofactor_method: TwoFactorMethod,
    pub twofactor_secret: Option<String>,
}

impl User {
    pub fn new(
        username: &str,
        pam_service: &str,
        method: TwoFactorMethod,
        secret: Option<String>,
    ) -> Self {
        Self {
            username: username.to_string(),
            pam_service: pam_service.to_string(),
            twofactor_method: method,
            twofactor_secret: secret,
        }
    }

    pub fn authenticate(&self, password: &str) -> bool {
        let mut context = Context::new(
            &self.pam_service,
            Some(&self.username),
            Conversation::with_credentials(&self.username, password),
        ).expect("Failed to initialize PAM context");

        context.authenticate(Flag::NONE).is_ok()
    }

    pub fn verify_2fa(&mut self, code: &str, _config_path: &Path) -> bool {
        match &mut self.twofactor_method {
            TwoFactorMethod::TOTP => {
                if let Some(secret) = &self.twofactor_secret {
                    let totp = TOTP::new(secret);
                    // TOTP.generate() in 0.5.1 requires period and timestamp
                    let period = 30; // typical 30s period
                    let timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    let current_code = totp.generate(period, timestamp);
                    return current_code.to_string() == code;
                }
                false
            }
            TwoFactorMethod::HOTP { counter } => {
                if let Some(secret) = &self.twofactor_secret {
                    let hotp = HOTP::new(secret);
                    let hotp_code = hotp.generate(*counter); // returns u32
                    if hotp_code.to_string() == code {
                        *counter += 1;
                        // TODO: save updated counter to config
                        return true;
                    }
                }
                false
            }
            TwoFactorMethod::None => true,
        }
    }
}
