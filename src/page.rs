use crate::{common::API_DOMAIN, errors};

pub async fn last_page() -> errors::Result<u32> {
    let url = format!("https://{}", API_DOMAIN);
    let resp = reqwest::get(&url)
        .await
        .map_err(|err| errors::Error::HttpRequest(err))?
        .error_for_status()
        .map_err(|err| errors::Error::HttpRequest(err))?;

    let content = resp
        .text()
        .await
        .map_err(|err| errors::Error::HttpRequest(err))?;

    let parser = libxml::parser::Parser::default_html();
    let document = parser
        .parse_string(content.as_bytes())
        .map_err(|err| errors::Error::HtmlParsing(err))?;

    let rootnode = document.get_root_element().unwrap();

    let query = r#"//div[@class = "next-page"]/a[last()]/text()"#;
    let pages = rootnode
        .findvalues(query)
        .map_err(|_| errors::Error::XpathQuerying(query.to_string()))?;

    if pages.is_empty() {
        let err = anyhow::anyhow!("Unexpected error: Page list fetch from {} is empty", url);
        return Err(errors::Error::Other(err));
    }

    let last_page = pages[0]
        .parse()
        .map_err(|err| anyhow::Error::from(err))
        .map_err(|err| errors::Error::Other(err))?;

    Ok(last_page)
}

pub async fn get_links(page: u32) -> errors::Result<Vec<String>> {
    let url = format!("https://{}/page-{}", API_DOMAIN, page);

    let resp = reqwest::get(&url)
        .await
        .map_err(|err| errors::Error::HttpRequest(err))?
        .error_for_status()
        .map_err(|err| errors::Error::HttpRequest(err))?;

    let content = resp
        .text()
        .await
        .map_err(|err| errors::Error::HttpRequest(err))?;

    let parser = libxml::parser::Parser::default_html();
    let document = parser
        .parse_string(content.as_bytes())
        .map_err(|err| errors::Error::HtmlParsing(err))?;

    let rootnode = document.get_root_element().unwrap();

    let query = r#"//ul[@class = "hsdn"]/li/h3/a/@href"#;
    let links = rootnode
        .findvalues(query)
        .map_err(|_| errors::Error::XpathQuerying(query.to_string()))?;

    if links.is_empty() {
        let err = anyhow::anyhow!("URL list fetch from {} is empty", url);
        return Err(errors::Error::Other(err));
    }

    Ok(links)
}