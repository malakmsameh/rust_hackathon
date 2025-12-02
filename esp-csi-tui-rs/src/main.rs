mod ui;
mod device;
mod visualization;
mod storage;
mod streaming;
mod models;
mod commands;

use anyhow::Result;
use std::panic;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let default_panic = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        default_panic(info);
        let _ = ui::cleanup_terminal();
    }));

    ui::app::run().await?;

    Ok(())
}
