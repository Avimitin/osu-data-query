use anyhow::{bail, Error};
use osu_query::{get_beatmaps, get_users, AppConfig};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opt = CommandLineOption::from_args();

    let cfg: AppConfig = confy::load("osu-query")?;

    match opt {
        CommandLineOption::GetBeatmap {
            mode,
            beatmap_set,
            beatmap_id,
        } => {
            if beatmap_set == "" && beatmap_id == "" {
                bail!("You need at lease give one argument about the song")
            }

            let resp = get_beatmaps(&cfg.api_key, &mode, &beatmap_set, &beatmap_id).await?;
            println!("{:?}", resp);
        }
        CommandLineOption::GetUser { user } => {
            let resp = get_users(&cfg.api_key, &user).await?;
            if resp.len() == 0 {
                bail!("No result for user: {}", user)
            }
            for u in resp {
                println!("{}", u);
            }
        }
    }

    Ok(())
}

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "A command line osu data fetcher")]
enum CommandLineOption {
    #[structopt(alias = "gb", about = "Get beatmap information")]
    GetBeatmap {
        #[structopt(short, long, default_value = "0", about = "Specify beatmap mode")]
        mode: String,

        #[structopt(short = "s", long, default_value = "")]
        beatmap_set: String,

        #[structopt(short = "b", long, default_value = "")]
        beatmap_id: String,
    },

    #[structopt(alias = "gu", about = "Get user information")]
    GetUser {
        #[structopt(short = "u", long, default_value = "")]
        user: String,
    },
}
