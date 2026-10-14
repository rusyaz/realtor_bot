use teloxide::types::{ButtonRequest, KeyboardButton, KeyboardMarkup};

use crate::texts::*;

fn rows(rows: &[&[&str]]) -> KeyboardMarkup {
    KeyboardMarkup::new(
        rows.iter()
            .map(|row| row.iter().map(|t| KeyboardButton::new(*t)).collect::<Vec<_>>()),
    )
    .resize_keyboard()
}

pub fn main_menu() -> KeyboardMarkup {
    rows(&[&[BTN_APPLY], &[BTN_SUPPORT]])
}

pub fn yes_no() -> KeyboardMarkup {
    rows(&[&[BTN_YES, BTN_NO]])
}

pub fn regions() -> KeyboardMarkup {
    rows(&[&REGIONS])
}

pub fn rooms() -> KeyboardMarkup {
    rows(&[&[STUDIO, "1", "2"], &["3", "4", ROOMS_MAX]])
}

pub fn repairs() -> KeyboardMarkup {
    rows(&[&REPAIRS])
}

pub fn photos_done() -> KeyboardMarkup {
    rows(&[&[BTN_DONE]])
}

pub fn send_phone() -> KeyboardMarkup {
    KeyboardMarkup::new([[KeyboardButton::new(BTN_SEND_PHONE).request(ButtonRequest::Contact)]])
        .resize_keyboard()
}
