//! /start, support, main menu (State::MainMenu).

use teloxide::prelude::*;

use super::{HandlerResult, MyDialogue, ask};
use crate::{config::Config, keyboards, state::State, texts};

pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dialogue.update(State::MainMenu).await?;
    ask(&bot, &msg, texts::WELCOME, Some(keyboards::main_menu())).await
}

pub async fn support(bot: Bot, msg: Message, config: Config) -> HandlerResult {
    bot.send_message(msg.chat.id, format!("{}{}", texts::SUPPORT_PREFIX, config.support))
        .await?;
    Ok(())
}

/// Main menu: one arm per button. To add a feature — add a button in
/// `texts.rs` + `keyboards::main_menu()`, an arm here, and its states.
pub async fn menu(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text().map(str::trim) {
        Some(texts::BTN_APPLY) => {
            dialogue.update(State::Owner).await?;
            ask(&bot, &msg, texts::ASK_OWNER, Some(keyboards::yes_no())).await
        }
        _ => ask(&bot, &msg, texts::USE_MENU, Some(keyboards::main_menu())).await,
    }
}
