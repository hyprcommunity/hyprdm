use libc;
use pam_client::{Context, Flag};
use pam_client::conv_mock::Conversation;
use otpauth::{TOTP, HOTP};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;
use std::ffi::OsString;

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
    fn detect_system_username() -> String {
        // Öncelikle $USER ortam değişkeni
        if let Ok(u) = env::var("USER") {
            if !u.is_empty() {
                return u;
            }
        }

        // $LOGNAME yedeği
        if let Ok(u) = env::var("LOGNAME") {
            if !u.is_empty() {
                return u;
            }
        }

        #[cfg(unix)]
        {
            use libc::{getpwuid, geteuid};
            use std::ffi::CStr;
            unsafe {
                let pw = getpwuid(geteuid());
                if !pw.is_null() {
                    let name = CStr::from_ptr((*pw).pw_name);
                    if let Ok(s) = name.to_str() {
                        return s.to_string();
                    }
                }
            }
        }

        "unknown".to_string()
    }

    pub fn new(
        username: &str,
        pam_service: &str,
        method: TwoFactorMethod,
        secret: Option<String>,
    ) -> Self {
        let uname = if username.is_empty() || username == "user" {
            Self::detect_system_username()
        } else {
            username.to_string()
        };

        // 🔧 PAM servisini doğrula — yanlış/boş isim gelirse "system-login" kullan.
        let pam = if pam_service.is_empty()
            || pam_service == "login"
            || pam_service == "login_service"
            || pam_service == "default"
        {
            "system-login".to_string()
        } else {
            pam_service.to_string()
        };

        Self {
            username: uname,
            pam_service: pam,
            twofactor_method: method,
            twofactor_secret: secret,
        }
    }

    pub fn authenticate(&self, password: &str) -> bool {
        println!(
            "[HyprDM PAM] Authenticating user='{}' via service='{}'",
            self.username, self.pam_service
        );

        let mut context = Context::new(
            &self.pam_service,
            Some(&self.username),
            Conversation::with_credentials(&self.username, password),
        )
        .unwrap_or_else(|_| panic!("Failed to initialize PAM context for {}", self.username));

        context.authenticate(Flag::NONE).is_ok()
    }

    pub fn verify_2fa(&mut self, code: &str, _config_path: &Path) -> bool {
        match &mut self.twofactor_method {
            TwoFactorMethod::TOTP => {
                if let Some(secret) = &self.twofactor_secret {
                    let totp = TOTP::new(secret);
                    let period = 30;
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
                    let hotp_code = hotp.generate(*counter);
                    if hotp_code.to_string() == code {
                        *counter += 1;
                        return true;
                    }
                }
                false
            }
            TwoFactorMethod::None => true,
        }
    }
}
