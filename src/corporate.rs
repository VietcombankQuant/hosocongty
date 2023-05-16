use crate::{
    common::{Coporate, API_DOMAIN},
    errors,
};

pub async fn get_corporate_info(url: String) -> errors::Result<Coporate> {
    let url = format!("https://{}/{}", API_DOMAIN, url);

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

    let query = r#"//ul[@class = "hsct"]/li"#;
    let elements = rootnode
        .findnodes(query)
        .map_err(|_| errors::Error::XpathQuerying(query.to_string()))?;

    let results = elements
        .into_iter()
        .filter_map(|element| -> Option<(String, String)> {
            let mut labels = match element.findvalues("./label//text()") {
                Ok(labels) => labels,
                Err(_) => return None,
            };

            let label = match labels.pop() {
                Some(label) => label,
                None => return None,
            };

            let mut values = match element.findvalues("./span//text()") {
                Ok(values) => values,
                Err(_) => return None,
            };

            let value = match values.pop() {
                Some(value) => value,
                None => return None,
            };

            Some((label, value))
        })
        .collect::<Coporate>();

    Ok(results)
}
