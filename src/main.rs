use std::{collections::HashMap, println};

use futures::{StreamExt, TryStreamExt};

mod common;
mod corporate;
mod errors;
mod page;

async fn get_corporate_info_from_page(page: u32) -> errors::Result<Vec<HashMap<String, String>>> {
    let links = page::get_links(page).await?;
    let corporates = futures::stream::iter(links.into_iter())
        .map(corporate::get_corporate_info)
        .buffer_unordered(12)
        .try_collect::<Vec<HashMap<_, _>>>()
        .await?;

    println!("{:#?}", corporates);
    Ok(corporates)
}

fn setup_logger() {
    let env = env_logger::Env::new()
        .default_filter_or("info")
        .default_write_style_or("auto");
    env_logger::init_from_env(env);
}

#[tokio::main]
async fn main() -> Result<(), errors::Error> {
    setup_logger();
    //let last_page = last_page().await?;
    let last_page: u32 = 2;
    let _ = futures::stream::iter(1..=last_page)
        .map(get_corporate_info_from_page)
        .buffer_unordered(2)
        .try_collect::<Vec<_>>()
        .await?;

    Ok(())
}
