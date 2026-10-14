mod config;
mod handlers;
mod keyboards;
mod notify;
mod state;
mod texts;

use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

use crate::{config::Config, state::State};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();

    let config = Config::from_env();
    let bot = Bot::from_env();
    log::info!("bot started");

    Dispatcher::builder(bot, handlers::schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new(), config])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
