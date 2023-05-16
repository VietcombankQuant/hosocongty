use std::{ops::Deref, sync::Arc};

use crate::common::Coporate;
use futures::StreamExt;
use tokio_stream::wrappers::UnboundedReceiverStream;

mod common;
mod corporate;
mod page;
mod results;
mod utils;

fn save_corporate_info(storage: &rocksdb::DB, corporate: &Coporate) -> results::Result<()> {
    let key = match corporate.get("TaxCode") {
        Some(key) => key,
        None => {
            let err = anyhow::anyhow!(
                "Failed to get `TaxCode` key from corporate info {:?}",
                corporate
            );
            return Err(results::Error::Other(err));
        }
    };
    let value = serde_json::to_string(&corporate)
        .map_err(|err| results::Error::Other(anyhow::Error::from(err)))?;

    storage
        .put(key, &value)
        .map_err(|err| results::Error::Other(anyhow::Error::from(err)))?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), results::Error> {
    utils::setup_logger();

    // Setup Database
    let db_path = "storage";
    let storage = rocksdb::DB::open_default(db_path)
        .map_err(|err| results::Error::Other(anyhow::Error::from(err)))?;
    let storage = Arc::new(storage);

    // Setup streams
    let last_page = page::last_page().await?;
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
    tokio::spawn(async move { page::get_links(1..=last_page, sender).await });
    UnboundedReceiverStream::new(receiver)
        .map(corporate::get_corporate_info)
        .buffer_unordered(64)
        .map(move |result| (result, storage.clone()))
        .map(|(result, storage)| match result {
            Err(err) => {
                log::error!("{}", err);
            }
            Ok(corporate) => {
                if let Err(err) = save_corporate_info(&storage.deref(), &corporate) {
                    log::error!("{}", err);
                }

                log::info!(
                    "Successfully save corporate info {:?} to database",
                    corporate
                );
            }
        })
        .collect::<()>()
        .await;

    Ok(())
}
