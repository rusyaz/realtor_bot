//! Text form steps: Owner → Region → Address → Floor → Area → Rooms → Repair.
//! Each handler validates the answer, saves it into `lead`, moves to the next
//! state and asks the next question. On bad input — re-asks, state unchanged.

use teloxide::prelude::*;

use super::{HandlerResult, MyDialogue, ask};
use crate::{
    keyboards,
    state::{Lead, State},
    texts,
};

fn text(msg: &Message) -> Option<&str> {
    msg.text().map(str::trim).filter(|t| !t.is_empty())
}

pub async fn owner(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let owner = match text(&msg) {
        Some(texts::BTN_YES) => true,
        Some(texts::BTN_NO) => false,
        _ => return ask(&bot, &msg, texts::ERR_YES_NO, Some(keyboards::yes_no())).await,
    };
    let lead = Lead { owner, ..Default::default() };
    dialogue.update(State::Region { lead }).await?;
    ask(&bot, &msg, texts::ASK_REGION, Some(keyboards::regions())).await
}

pub async fn region(bot: Bot, dialogue: MyDialogue, msg: Message, mut lead: Lead) -> HandlerResult {
    let Some(r) = text(&msg).filter(|t| texts::REGIONS.contains(t)) else {
        return ask(&bot, &msg, texts::ERR_USE_BUTTONS, Some(keyboards::regions())).await;
    };
    lead.region = r.to_owned();
    dialogue.update(State::Address { lead }).await?;
    ask(&bot, &msg, texts::ASK_ADDRESS, None).await
}

pub async fn address(bot: Bot, dialogue: MyDialogue, msg: Message, mut lead: Lead) -> HandlerResult {
    let Some(a) = text(&msg) else {
        return ask(&bot, &msg, texts::ERR_ADDRESS, None).await;
    };
    lead.address = a.to_owned();
    dialogue.update(State::Floor { lead }).await?;
    ask(&bot, &msg, texts::ASK_FLOOR, None).await
}

pub async fn floor(bot: Bot, dialogue: MyDialogue, msg: Message, mut lead: Lead) -> HandlerResult {
    // "5", "5/17", "-1" are fine.
    let valid = |t: &&str| t.len() <= 10 && t.chars().all(|c| c.is_ascii_digit() || c == '/' || c == '-');
    let Some(f) = text(&msg).filter(valid) else {
        return ask(&bot, &msg, texts::ERR_FLOOR, None).await;
    };
    lead.floor = f.to_owned();
    dialogue.update(State::Area { lead }).await?;
    ask(&bot, &msg, texts::ASK_AREA, None).await
}

pub async fn area(bot: Bot, dialogue: MyDialogue, msg: Message, mut lead: Lead) -> HandlerResult {
    let parsed = text(&msg)
        .and_then(|t| t.replace(',', ".").parse::<f32>().ok())
        .filter(|a| (5.0..=1000.0).contains(a));
    let Some(a) = parsed else {
        return ask(&bot, &msg, texts::ERR_AREA, None).await;
    };
    lead.area = a;
    dialogue.update(State::Rooms { lead }).await?;
    ask(&bot, &msg, texts::ASK_ROOMS, Some(keyboards::rooms())).await
}

pub async fn rooms(bot: Bot, dialogue: MyDialogue, msg: Message, mut lead: Lead) -> HandlerResult {
    let parsed = match text(&msg) {
        Some(texts::STUDIO) => Some(0),
        Some(texts::ROOMS_MAX) => Some(5),
        Some(t) => t.parse::<u8>().ok().filter(|n| (1..=20).contains(n)),
        None => None,
    };
    let Some(n) = parsed else {
        return ask(&bot, &msg, texts::ERR_USE_BUTTONS, Some(keyboards::rooms())).await;
    };
    lead.rooms = n;
    dialogue.update(State::Repair { lead }).await?;
    ask(&bot, &msg, texts::ASK_REPAIR, Some(keyboards::repairs())).await
}

pub async fn repair(bot: Bot, dialogue: MyDialogue, msg: Message, mut lead: Lead) -> HandlerResult {
    let Some(r) = text(&msg).filter(|t| texts::REPAIRS.contains(t)) else {
        return ask(&bot, &msg, texts::ERR_USE_BUTTONS, Some(keyboards::repairs())).await;
    };
    lead.repair = r.to_owned();
    dialogue.update(State::Photos { lead }).await?;
    ask(&bot, &msg, &texts::ask_photos(super::photos::MIN_PHOTOS), None).await
}
