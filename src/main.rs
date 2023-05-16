use futures::StreamExt;
use tokio_stream::wrappers::UnboundedReceiverStream;

mod common;
mod corporate;
mod errors;
mod page;

fn setup_logger() {
    let env = env_logger::Env::new()
        .default_filter_or("info")
        .default_write_style_or("auto");
    env_logger::init_from_env(env);
}

#[tokio::main]
async fn main() -> Result<(), errors::Error> {
    setup_logger();

    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
    tokio::spawn(async move { page::get_links(1..=2, sender).await });
    UnboundedReceiverStream::new(receiver)
        .map(corporate::get_corporate_info)
        .buffer_unordered(8)
        .map(|result| match result {
            Err(err) => {
                log::error!("{}", err);
            }
            Ok(corporate) => {
                log::info!("{:#?}", corporate);
            }
        })
        .collect::<()>()
        .await;

    Ok(())
}
