use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};
use std::io;
use anyhow::Result;

pub fn setup_terminal() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    Ok(())
}

pub fn cleanup_terminal() -> Result<()> {
    disable_raw_mode()?;
    Ok(())
}
