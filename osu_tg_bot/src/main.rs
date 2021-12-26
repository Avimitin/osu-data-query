use anyhow::{Context, Result};
use lazy_static::lazy_static;
use osu_query::{beatmaps::BeatMap, prelude::*};
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

fn pretty_beatmap_style(bmp: &[BeatMap]) -> String {
  if bmp.is_empty() {
    return String::from("No beatmap");
  }

  // Use the first map
  format!(
    "🎵 Beatmap Title: {}\n🤵 Artist: {}\n🔍 Set ID: {}\n⭐️ Difficulty: {}\n📍 BID: {}",
    bmp[0].title_unicode,
    bmp[0].artist_unicode,
    bmp[0].beatmapset_id,
    bmp[0].stars,
    bmp[0].beatmap_id,
  )
}

async fn get_beatmap(cx: &Cxm, link: &str) -> Result<()> {
  let msg = cx
    .answer(format!("Searching information for {}", link))
    .await
    .with_context(|| "Fail to send get beatmap response".to_string())?;

  match get_beatmaps_from_link(&APP_CONFIG.api_key, link).await {
    Ok(bmp) => {
      cx.requester
        .edit_message_text(msg.chat_id(), msg.id, pretty_beatmap_style(&bmp))
        .await
        .with_context(|| "Fail to send beatmaps information back".to_string())?;
    }
    Err(e) => {
      cx.requester
        .edit_message_text(msg.chat_id(), msg.id, format!("{}", e))
        .await
        .with_context(|| "Fail to send beatmaps information back".to_string())?;
    }
  }

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

  let bot_name: String = "OSU Bot".to_string();
  teloxide::commands_repl(bot, bot_name, answer).await;
}
