//! Sending a finished lead to the realtor.

use teloxide::{
    prelude::*,
    types::{InputFile, InputMedia, InputMediaPhoto, ParseMode},
    utils::html,
};

use crate::{config::Config, handlers::HandlerResult, state::Lead};

pub async fn send_to_realtor(bot: &Bot, config: &Config, msg: &Message, lead: &Lead, phone: &str) -> HandlerResult {
    // sendMediaGroup takes 2..=10 items, so a trailing single photo goes via sendPhoto.
    for chunk in lead.photos.chunks(10) {
        if let [single] = chunk {
            bot.send_photo(config.realtor, InputFile::file_id(single.clone())).await?;
        } else {
            let media = chunk
                .iter()
                .map(|id| InputMedia::Photo(InputMediaPhoto::new(InputFile::file_id(id.clone()))));
            bot.send_media_group(config.realtor, media).await?;
        }
    }

    bot.send_message(config.realtor, format_lead(msg, lead, phone))
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

fn format_lead(msg: &Message, lead: &Lead, phone: &str) -> String {
    // Works even without @username — falls back to a tg://user link.
    let tg = msg
        .from
        .as_ref()
        .map(html::user_mention_or_link)
        .unwrap_or_else(|| "—".into());
    let rooms = if lead.rooms == 0 { "студия".to_owned() } else { lead.rooms.to_string() };

    // User input is escaped: an address with `<` would otherwise break HTML.
    format!(
        "🏠 <b>Новая заявка: сдать квартиру</b>\n\n\
         <b>Собственник:</b> {}\n\
         <b>Регион:</b> {}\n\
         <b>Адрес:</b> {}\n\
         <b>Этаж:</b> {}\n\
         <b>Площадь:</b> {} м²\n\
         <b>Комнат:</b> {}\n\
         <b>Ремонт:</b> {}\n\
         <b>Фото:</b> {} шт.\n\n\
         <b>Телефон:</b> {}\n\
         <b>Telegram:</b> {}",
        if lead.owner { "да" } else { "нет" },
        html::escape(&lead.region),
        html::escape(&lead.address),
        html::escape(&lead.floor),
        lead.area,
        rooms,
        html::escape(&lead.repair),
        lead.photos.len(),
        html::escape(phone),
        tg,
    )
}
