//! All user-facing text in one place — edit here when the client asks.

// Buttons
pub const BTN_APPLY: &str = "📝 Сдать квартиру";
pub const BTN_SUPPORT: &str = "🆘 Поддержка";
pub const BTN_DONE: &str = "✅ Готово";
pub const BTN_SEND_PHONE: &str = "📱 Отправить номер";
pub const BTN_YES: &str = "Да";
pub const BTN_NO: &str = "Нет";

pub const REGIONS: [&str; 2] = ["Москва", "Московская область"];
pub const REPAIRS: [&str; 3] = ["Косметический", "Евро", "Дизайнерский"];
pub const STUDIO: &str = "Студия";
pub const ROOMS_MAX: &str = "5+";

// Menu
pub const WELCOME: &str = "Здравствуйте! Этот бот принимает заявки от тех, кто хочет сдать квартиру \
    в Москве или Московской области.\n\n\
    Отправляя заявку, вы соглашаетесь на обработку персональных данных.";
pub const USE_MENU: &str = "Выберите действие в меню ниже.";
pub const SUPPORT_PREFIX: &str = "По любым вопросам пишите: @";

// Questions
pub const ASK_OWNER: &str = "Вы собственник квартиры?";
pub const ASK_REGION: &str = "Где находится квартира?";
pub const ASK_ADDRESS: &str = "Укажите адрес квартиры (город, улица, дом).";
pub const ASK_FLOOR: &str = "Этаж? Например: 5 или 5/17";
pub const ASK_AREA: &str = "Площадь в м²? Например: 42.5";
pub const ASK_ROOMS: &str = "Сколько комнат?";
pub const ASK_REPAIR: &str = "Какой ремонт?";
pub fn ask_photos(min: usize) -> String {
    format!("Пришлите фото квартиры — минимум {min}. Можно альбомом.")
}
pub const PHOTOS_ENOUGH: &str = "Фото получены. Можно добавить ещё или нажать «Готово».";
pub const ASK_PHONE: &str = "Оставьте номер телефона — нажмите кнопку или напишите его.";
pub const DONE: &str = "Спасибо! Заявка отправлена, риэлтор свяжется с вами в ближайшее время.";

// Validation errors
pub const ERR_YES_NO: &str = "Нажмите «Да» или «Нет».";
pub const ERR_USE_BUTTONS: &str = "Выберите вариант кнопкой.";
pub const ERR_ADDRESS: &str = "Напишите адрес текстом.";
pub const ERR_FLOOR: &str = "Укажите этаж цифрами, например: 5 или 5/17";
pub const ERR_AREA: &str = "Укажите площадь числом, например: 42.5";
pub const ERR_PHONE: &str = "Не похоже на номер. Нажмите кнопку или напишите номер, например +7 999 123-45-67";

pub fn photos_left(left: usize) -> String {
    format!("Нужно ещё фото: {left}. Пришлите их как фото, не файлом.")
}
