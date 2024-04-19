use scraper::{Html, Selector};

/// start page
/// -> download
/// -> get links
/// -> repeat until
/// -> you find target page
///
/// https://it.wikipedia.org/wiki/Adolf_Hitler
///

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let start = "/wiki/Mela".to_string();
    let end = "/wiki/Adolf_Hitler".to_string();

    let mut pages = vec![start];

    while !pages.is_empty() {
        let current_page = pages.remove(0);
        dbg!(&current_page);

        let page_content = download_wiki_page(&current_page).await?;
        let mut new_links = get_links_from_page(&page_content);

        if new_links.contains(&end) {
            println!("VIVA");
            break;
        }

        pages.append(&mut new_links);
    }

    Ok(())
}

async fn download_wiki_page(rel_link: &str) -> anyhow::Result<String> {
    Ok(
        reqwest::get(format!("https://it.wikipedia.org{}", rel_link))
            .await?
            .text()
            .await?,
    )
}

fn get_links_from_page(html: &str) -> Vec<String> {
    let fragment = Html::parse_fragment(html);
    let a_selector = Selector::parse("#bodyContent a").unwrap();

    fragment
        .select(&a_selector)
        .filter_map(|e| e.value().attr("href").map(|e| e.to_string()))
        .filter(|e| e.starts_with("/wiki/") && !e.contains(":"))
        .collect()
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn it_downloads_wiki_page() {
        assert!(download_wiki_page("/wiki/Adolf_Hitler")
            .await
            .unwrap()
            .contains("<html"));
    }

    #[tokio::test]
    async fn it_gets_links_from_page() {
        let page = download_wiki_page("/wiki/Adolf_Hitler").await.unwrap();

        assert_eq!(get_links_from_page(&page)[0], "/wiki/Hitler_(disambigua)");
    }
}
