use std::env;

pub struct AppConfig {
    pub jwt_secret: String,
    pub resend_key: String,
    pub resend_from_email: String,
    pub base_api_url: String,
    //pub access_token_ttl: Duration,
    //pub refresh_token_ttl: Duration,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let resend_key = env::var("RESEND_KEY").expect("RESEND_KEY must be set");

        let resend_from_email =
            env::var("RESEND_FROM_EMAIL").expect("RESEND_FROM_EMAIL must be set");

        let base_api_url = env::var("BASE_API_URL").expect("BASE_API_URL must be set");

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
            base_api_url,
            //access_token_ttl: Duration::seconds(access_token_ttl_secs),
            //refresh_token_ttl: Duration::days(refresh_token_ttl_days),
        }
    }
}
