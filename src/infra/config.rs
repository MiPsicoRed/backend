use std::env;

use tracing::{info, warn};

pub struct AppConfig {
    pub jwt_secret: String,
    pub resend_key: String,
    pub resend_from_email: String,
    pub base_frontend_url: String,
    pub release_mode: bool,
    pub polar_access_token: String,
    //pub access_token_ttl: Duration,
    //pub refresh_token_ttl: Duration,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let resend_key = env::var("RESEND_KEY").expect("RESEND_KEY must be set");

        let resend_from_email =
            env::var("RESEND_FROM_EMAIL").expect("RESEND_FROM_EMAIL must be set");

        let base_frontend_url =
            env::var("BASE_FRONTEND_URL").expect("BASE_FRONTEND_URL must be set");

        let release_mode: bool = match env::var("RELEASE_MODE") {
            Ok(val) => match val.parse::<bool>() {
                Ok(parsed) => parsed,
                Err(_) => {
                    warn!("RELEASE_MODE is set but not a valid boolean, defaulting to false");
                    false
                }
            },
            Err(_) => {
                info!("RELEASE_MODE not set, defaulting to false");
                false
            }
        };

        let polar_access_token =
            env::var("POLAR_ACCESS_TOKEN").expect("POLAR_ACCESS_TOKEN must be set");

        // let refresh_token_ttl_days: i64 = env::var("REFRESH_TOKEN_TTL_DAYS")
        //     .unwrap_or("30".to_string())
        //     .parse()
        //     .expect("REFRESH_TOKEN_TTL_DAYS must be a valid number");

        // let access_token_ttl_secs: i64 = env::var("ACCESS_TOKEN_TTL_SECS")
        //     .unwrap_or("30".to_string())
        //     .parse()
        //     .expect("ACCESS_TOKEN_TTL_SECS must be a valid number");

        Self {
            jwt_secret,
            resend_key,
            resend_from_email,
            base_frontend_url,
            release_mode,
            polar_access_token,
            //access_token_ttl: Duration::seconds(access_token_ttl_secs),
            //refresh_token_ttl: Duration::days(refresh_token_ttl_days),
        }
    }
}
