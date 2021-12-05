use teloxide::{prelude::*, utils::command::BotCommand};
use osu_query::prelude::*;
use std::error::Error;
use lazy_static::lazy_static;

lazy_static!{
    static ref APP_CONFIG: AppConfig = confy::load("osu-query").unwrap();
}

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a osu beatmap url")]
    GetBeatmap(String),
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).await?,
        Command::GetBeatmap(link) => {
            let bmps = get_beatmaps_from_link(&APP_CONFIG.api_key, &link).await?;
            cx.answer(format!("{:#?}", bmps)).await?
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("starting osu bot...");

    let bot = Bot::from_env().auto_send();

    let bot_name: String = format!("OSU Bot");
    teloxide::commands_repl(bot, bot_name, answer).await;
}
