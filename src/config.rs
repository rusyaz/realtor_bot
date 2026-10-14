use teloxide::types::ChatId;

/// Settings from `.env`. TELOXIDE_TOKEN is read by `Bot::from_env()` separately.
#[derive(Clone)]
pub struct Config {
    /// Where leads are sent (realtor's DM).
    pub realtor: ChatId,
    /// Username shown on "Support", without `@`.
    pub support: String,
}

impl Config {
    pub fn from_env() -> Self {
        let realtor = std::env::var("REALTOR_CHAT_ID")
            .expect("REALTOR_CHAT_ID not set")
            .parse()
            .expect("REALTOR_CHAT_ID must be i64");
        let support = std::env::var("SUPPORT_USERNAME").expect("SUPPORT_USERNAME not set");

        Self { realtor: ChatId(realtor), support }
    }
}
