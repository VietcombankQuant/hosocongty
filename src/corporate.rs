use crate::{
    common::{Coporate, API_DOMAIN},
    results,
};

pub async fn get_corporate_info(url: String) -> results::Result<Coporate> {
    let url = format!("https://{}/{}", API_DOMAIN, url);

    let resp = reqwest::get(&url)
        .await
        .map_err(|err| results::Error::HttpRequest(err))?
        .error_for_status()
        .map_err(|err| results::Error::HttpRequest(err))?;

    let content = resp
        .text()
        .await
        .map_err(|err| results::Error::HttpRequest(err))?;

    let parser = libxml::parser::Parser::default_html();
    let document = parser
        .parse_string(content.as_bytes())
        .map_err(|err| results::Error::HtmlParsing(err))?;

    let rootnode = document.get_root_element().unwrap();

    let query = r#"//ul[@class = "hsct"]/li"#;
    let elements = rootnode
        .findnodes(query)
        .map_err(|_| results::Error::XpathQuerying(query.to_string()))?;

    let mut results = elements
        .into_iter()
        .filter_map(|element| -> Option<(String, String)> {
            let label = match element.findvalues("./label//text()") {
                Ok(labels) => labels,
                Err(_) => return None,
            }
            .into_iter()
            .fold(String::default(), |result, elem| result + elem.trim());

            let value = match element.findvalues("./span//text()") {
                Ok(values) => values,
                Err(_) => return None,
            }
            .into_iter()
            .fold(String::default(), |result, elem| result + elem.trim());

            if label.is_empty() || value.is_empty() {
                return None;
            }

            Some((label, value))
        })
        .collect::<Coporate>();

    // Coporate Name
    let query = r#"//ul[@class = "hsct"]/li[1]/h1/text()"#;
    let corporate_name = rootnode
        .findvalues(query)
        .map_err(|_| results::Error::XpathQuerying(query.to_string()))?
        .concat();

    results.insert("Tên doanh nghiệp".to_string(), corporate_name);

    // Tax code
    let tax_code = match results.remove_entry("Mã số thuế:") {
        Some((_key, value)) => value,
        None => {
            let query = r#"//ul[@class = "hsct"]/li[./label/i[contains(@class, "fa-hashtag")]]/span/text()"#;
            let tax_code = rootnode
                .findvalues(query)
                .map_err(|_| results::Error::XpathQuerying(query.to_string()))?
                .concat();
            tax_code
        }
    };
    results.insert("TaxCode".to_string(), tax_code);

    // Last updated date
    let query = r#"//ul[@class = "hsct"]/li[./i[contains(@class, "fa-clock-o")]]/i[last()]/text()"#;
    let last_update = rootnode
        .findvalues(query)
        .map_err(|_| results::Error::XpathQuerying(query.to_string()))?
        .concat();
    if !last_update.is_empty() {
        results.insert("Ngày cập nhật cuối".to_string(), last_update);
    }

    // Decode Email

    Ok(results)
}
