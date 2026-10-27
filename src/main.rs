use anyhow::Ok;

use crate::model::get_actions_from_match_url;

mod model;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let actions = get_actions_from_match_url(
        "https://www.urbansoccer.fr/videos/1074/1083/26-10-2026/557136/",
    )
    .await?;
    println!("{:?}", actions);
    Ok(())
}
