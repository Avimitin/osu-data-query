use anyhow::{Context, Result};
use lazy_static::lazy_static;
use osu_query::prelude::*;
use teloxide::{prelude::*, utils::command::BotCommand};

lazy_static! {
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

type Cxm = UpdateWithCx<AutoSend<Bot>, Message>;

async fn get_beatmap(cx: &Cxm, link: &str) -> Result<()> {
    let bmps = get_beatmaps_from_link(&APP_CONFIG.api_key, link).await?;
    cx.answer(format!("{:#?}", bmps))
        .await
        .with_context(|| format!("Fail to send beatmaps back"))?;
    Ok(())
}

async fn print_help(cx: &Cxm) -> Result<()> {
    cx.answer(Command::descriptions()).await?;
    Ok(())
}

async fn answer(cx: Cxm, command: Command) -> Result<()> {
    match command {
        Command::Help => print_help(&cx).await?,
        Command::GetBeatmap(link) => get_beatmap(&cx, &link).await?,
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
