//! Routing: which handler gets a message, depending on the dialogue state.

mod form;
mod menu;
mod phone;
mod photos;

use std::error::Error;

use teloxide::{
    dispatching::{UpdateHandler, dialogue::InMemStorage},
    prelude::*,
    types::{KeyboardMarkup, KeyboardRemove},
};

use crate::{state::State, texts::BTN_SUPPORT};

pub type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;
pub type MyDialogue = Dialogue<State, InMemStorage<State>>;

pub fn schema() -> UpdateHandler<Box<dyn Error + Send + Sync + 'static>> {
    use dptree::case;

    Update::filter_message()
        .inspect(|msg: Message| log::info!("message from chat_id={}", msg.chat.id))
        .enter_dialogue::<Message, InMemStorage<State>, State>()
        // Commands work in any state.
        .branch(dptree::filter(|msg: Message| is_text(&msg, "/start")).endpoint(menu::start))
        .branch(
            dptree::filter(|msg: Message| is_text(&msg, "/support") || is_text(&msg, BTN_SUPPORT))
                .endpoint(menu::support),
        )
        // One handler per state.
        .branch(case![State::MainMenu].endpoint(menu::menu))
        .branch(case![State::Owner].endpoint(form::owner))
        .branch(case![State::Region { lead }].endpoint(form::region))
        .branch(case![State::Address { lead }].endpoint(form::address))
        .branch(case![State::Floor { lead }].endpoint(form::floor))
        .branch(case![State::Area { lead }].endpoint(form::area))
        .branch(case![State::Rooms { lead }].endpoint(form::rooms))
        .branch(case![State::Repair { lead }].endpoint(form::repair))
        .branch(case![State::Photos { lead }].endpoint(photos::photos))
        .branch(case![State::Phone { lead }].endpoint(phone::phone))
}

pub fn is_text(msg: &Message, expected: &str) -> bool {
    msg.text().map(str::trim) == Some(expected)
}

/// Send a message; `None` hides the keyboard (for free-text answers).
pub async fn ask(bot: &Bot, msg: &Message, text: &str, kb: Option<KeyboardMarkup>) -> HandlerResult {
    let req = bot.send_message(msg.chat.id, text);
    match kb {
        Some(kb) => req.reply_markup(kb).await?,
        None => req.reply_markup(KeyboardRemove::new()).await?,
    };
    Ok(())
}
