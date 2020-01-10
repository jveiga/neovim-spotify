mod lyrics;
mod messages;
mod neovim;
mod spotify;

#[async_std::main]
async fn main() {
    if let Some(mut nvim) = neovim::EventHandler::new() {
        // Block
        nvim.handle_events().await;
    }
}
