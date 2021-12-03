use anyhow::Error;
use osu_query::{get_beatmaps, AppConfig};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cfg: AppConfig = confy::load("osu-query")?;

    let resp = get_beatmaps(&cfg.api_key, "", "", "1872396").await?;

    println!("{:?}", resp);
    Ok(())
}
