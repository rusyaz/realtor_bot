//! State::Photos — collect photos until the user presses "Done".

use teloxide::prelude::*;

use super::{HandlerResult, MyDialogue, ask, is_text};
use crate::{
    keyboards,
    state::{Lead, State},
    texts,
};

pub const MIN_PHOTOS: usize = 5;
const MAX_PHOTOS: usize = 30;

// An album arrives as separate updates, one photo each. The dispatcher handles
// one chat sequentially, so pushing into state one by one is race-free.
pub async fn photos(bot: Bot, dialogue: MyDialogue, msg: Message, mut lead: Lead) -> HandlerResult {
    if let Some(sizes) = msg.photo() {
        if lead.photos.len() >= MAX_PHOTOS {
            return Ok(());
        }
        // `sizes` = one photo in several resolutions; last is the largest.
        if let Some(largest) = sizes.last() {
            lead.photos.push(largest.file.id.clone());
        }
        let count = lead.photos.len();
        dialogue.update(State::Photos { lead }).await?;
        // Reply once when the minimum is reached, not on every photo of an album.
        if count == MIN_PHOTOS {
            ask(&bot, &msg, texts::PHOTOS_ENOUGH, Some(keyboards::photos_done())).await?;
        }
        return Ok(());
    }

    if is_text(&msg, texts::BTN_DONE) && lead.photos.len() >= MIN_PHOTOS {
        dialogue.update(State::Phone { lead }).await?;
        return ask(&bot, &msg, texts::ASK_PHONE, Some(keyboards::send_phone())).await;
    }

    let left = MIN_PHOTOS.saturating_sub(lead.photos.len());
    ask(&bot, &msg, &texts::photos_left(left), None).await
}
