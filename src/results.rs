use libxml::parser::XmlParseError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to request with error {0:?}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("failed to parse HTML document with error {0:?}")]
    HtmlParsing(#[from] XmlParseError),

    #[error("failed to get document nodes from XPATH query {0:?}")]
    XpathQuerying(String),

    #[error("{0:?}")]
    Other(anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
