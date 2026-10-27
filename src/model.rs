use anyhow::Ok;

#[derive(Debug)]
pub struct Action {
    pub video_url_1: String,
    pub video_url_2: Option<String>,
    pub time: String,
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
    let mut actions_iter = videos.map(|el| -> anyhow::Result<(String, String)> {
        let selector = scraper::Selector::parse("source")?;
        let mut source = el.select(&selector);
        let src = source
            .next()
            .ok_or(anyhow::anyhow!("Error when get first source"))?
            .attr("src")
            .ok_or(anyhow::anyhow!("Error when get src from source"))?;

        let selector = scraper::Selector::parse(".titre")?;
        let mut titre = el.select(&selector);
        let time = titre
            .next()
            .ok_or(anyhow::anyhow!("Error when get first title"))?
            .text()
            .next()
            .ok_or(anyhow::anyhow!("Error when get title content"))?;
        Ok((src.to_string(), time.to_string()))
    });
    match (actions_iter.next(), actions_iter.next()) {
        (Some(t1), None) => {
            let (video_url_1, time_1) = t1?;
            Ok(Action {
                video_url_1,
                video_url_2: None,
                time: time_1,
            })
        }
        (Some(t1), Some(t2)) => {
            let (video_url_1, time_1) = t1?;
            let (video_url_2, time_2) = t2?;
            if time_1 != time_2 {
                panic!("Time is different");
            }
            Ok(Action {
                video_url_1,
                video_url_2: Some(video_url_2),
                time: time_1,
            })
        }
        _ => Err(anyhow::anyhow!("No action found")),
    }
}
