use common::API_DOMAIN;

mod common;
mod errors;

#[tokio::main]
async fn main() -> Result<(), errors::Error> {
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

    println!("{:#?}", pages);

    Ok(())
}
