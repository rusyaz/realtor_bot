//! State::Phone — last step: take the phone, send the lead, back to menu.

use teloxide::prelude::*;

use super::{HandlerResult, MyDialogue, ask};
use crate::{
    config::Config,
    keyboards, notify,
    state::{Lead, State},
    texts,
};

pub async fn phone(bot: Bot, dialogue: MyDialogue, msg: Message, lead: Lead, config: Config) -> HandlerResult {
    let phone = match (msg.contact(), msg.text()) {
        (Some(c), _) => Some(c.phone_number.clone()),
        (None, Some(t)) if t.chars().filter(char::is_ascii_digit).count() >= 10 => Some(t.trim().to_owned()),
        _ => None,
    };
    let Some(phone) = phone else {
        return ask(&bot, &msg, texts::ERR_PHONE, None).await;
    };

    notify::send_to_realtor(&bot, &config, &msg, &lead, &phone).await?;

    dialogue.update(State::MainMenu).await?;
    ask(&bot, &msg, texts::DONE, Some(keyboards::main_menu())).await
}
