use anyhow::Ok;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

#[derive(Debug)]
pub struct Action {
    pub video_url_1: String,
    pub video_url_2: Option<String>,
    pub datetime: NaiveDateTime,
}

pub async fn get_actions_from_match_url(match_url: &str) -> anyhow::Result<Vec<Action>> {
    let match_html_str = reqwest::get(match_url).await?.text().await?;
    let links: Vec<String> = {
        let match_html = scraper::Html::parse_document(&match_html_str);
        let selector = scraper::Selector::parse(".videos-listint-item>a")?;
        match_html
            .select(&selector)
            .map(|el| el.attr("href").map(str::to_string))
            .collect::<Option<Vec<_>>>()
            .ok_or(anyhow::anyhow!(
                "Error when get href attribute for match_html"
            ))?
    };
    let futures = links.iter().filter_map(|link| {
        (link.split("/").count() == 7).then_some(get_actions_from_action_url(link))
    });
    Ok(futures::future::try_join_all(futures).await?)
}

async fn get_actions_from_action_url(action_url: &str) -> anyhow::Result<Action> {
    let action_html_str = reqwest::get(format!("https://www.urbansoccer.fr/{}", action_url))
        .await?
        .text()
        .await?;

    let mut url_splited_iter = action_url.split("/");
    let date = url_splited_iter
        .nth(4)
        .ok_or(anyhow::anyhow!("Date not found"))?;
    let time = url_splited_iter.nth(1).ok_or_else(|| {
        println!("action url failed: {}", action_url);
        anyhow::anyhow!("Time not found")
    })?;
    let date = NaiveDate::parse_from_str(date, "%d-%m-%Y")?;
    let time = NaiveTime::parse_from_str(time, "%H:%M:%S")?;
    let datetime = NaiveDateTime::new(date, time);

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
                datetime,
            })
        }
        (Some(url_1), Some(url_2)) => {
            let url_1 = url_1?;
            let url_2 = url_2?;

            Ok(Action {
                video_url_1: url_1.to_string(),
                video_url_2: Some(url_2.to_string()),
                datetime,
            })
        }
        _ => Err(anyhow::anyhow!("No action found")),
    }
}
