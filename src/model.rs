use anyhow::Ok;

#[derive(Debug)]
pub struct Action {
    pub video_url_1: String,
    pub video_url_2: Option<String>,
}

pub async fn get_actions_from_match_url(match_url: &str) -> anyhow::Result<Vec<Action>> {
    let match_html_str = reqwest::get(match_url).await?.text().await?;
    let match_html = scraper::Html::parse_document(&match_html_str);
    let selector = scraper::Selector::parse(".videos-listint-item>a")?;
    let elements = match_html.select(&selector);
    let links = elements
        .map(|el| el.attr("href"))
        .collect::<Option<Vec<&str>>>()
        .ok_or(anyhow::anyhow!(
            "Error when get href attribute for match_html"
        ))?;
    let futures = links
        .iter()
        .map(|&link| async move { get_actions_from_action_url(link).await });
    Ok(futures::future::try_join_all(futures).await?)
}

async fn get_actions_from_action_url(action_url: &str) -> anyhow::Result<Action> {
    let action_html_str = reqwest::get(format!("https://www.urbansoccer.fr/{}", action_url))
        .await?
        .text()
        .await?;
    let action_html = scraper::Html::parse_document(&action_html_str);
    let selector = scraper::Selector::parse("video")?;
    let videos = action_html.select(&selector);
    let mut actions_iter = videos.map(|el| {
        let selector = scraper::Selector::parse("source")?;
        let mut source = el.select(&selector);
        let src = source
            .next()
            .ok_or(anyhow::anyhow!("Error when get first source"))?
            .attr("src")
            .ok_or(anyhow::anyhow!("Error when get src from source"))?;
        Ok(src)
    });
    match (actions_iter.next(), actions_iter.next()) {
        (Some(url), None) => {
            let url = url?;
            Ok(Action {
                video_url_1: url.to_string(),
                video_url_2: None,
            })
        }
        (Some(url_1), Some(url_2)) => {
            let url_1 = url_1?;
            let url_2 = url_2?;

            Ok(Action {
                video_url_1: url_1.to_string(),
                video_url_2: Some(url_2.to_string()),
            })
        }
        _ => Err(anyhow::anyhow!("No action found")),
    }
}
