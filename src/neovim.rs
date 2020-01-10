use neovim_lib::{Neovim, NeovimApi, Session};
use std::error::Error;

use crate::lyrics;
use crate::messages::Messages;
use crate::spotify::{Spotify, SpotifyAPI};

/// EventHandler receives RPC requests, and maps them to right Spotify and Neovim commands.
pub struct EventHandler {
    nvim: Neovim,
    spotify: Box<dyn SpotifyAPI>,
}

impl EventHandler {
    pub fn new() -> Option<EventHandler> {
        let mut session = Session::new_parent().ok()?;
        session.set_infinity_timeout();
        let nvim = Neovim::new(session);
        let spotify = Spotify::new();

        Some(EventHandler {
            nvim,
            spotify: Box::new(spotify),
        })
    }

    pub async fn handle_events(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        for (event, _values) in receiver {
            match Messages::from(event) {
                Messages::CurrentSong => {
                    self.spotify.current_song().map(|song| {
                        let _ = self.nvim.command(&format!("echo \"{}\"", song));
                    });
                }

                Messages::PlayTrack(track) => {
                    self.spotify.play_track(&track);
                }

                Messages::PlayPause => {
                    self.spotify.play_pause();
                }

                Messages::Play => {
                    self.spotify.play();
                }

                Messages::Pause => {
                    self.spotify.pause();
                }

                Messages::Next => {
                    self.spotify.next();
                }

                Messages::Previous => {
                    self.spotify.previous();
                }

                Messages::Lyrics => {
                    if let Some(song) = self.spotify.current_song() {
                        let parts : Vec<&str> = song.split('-').collect();
                        if parts.len() < 2{
                            break;
                        }
                        let (artist, song) =
                            (parts[0].trim(), parts[0].trim());

                        if let Some(lyrics) = lyrics::find_lyrics(artist, song).await {
                            let lyrics_vec: Vec<String> =
                                lyrics.split('\n').map(|s| s.to_owned()).collect();
                            let _ = self.fill_buffer(lyrics_vec);
                        } else {
                            let _ = self
                                .nvim
                                .command(&format!("echo \"Could not find lyrics\""))
                                .map_err(|err| eprintln!("failed to echo {}", err));
                        }
                    }
                }

                // Handle any "Unknown" messages.
                Messages::Unknown(ev) => {
                    self.nvim
                        .command(&format!("echoerr \"{}\" Unknown command", ev))
                        .unwrap_or_else(|err| eprintln!("{}", err));
                }
            }
        }
    }

    fn fill_buffer(&mut self, lyrics: Vec<String>) -> Result<(), Box<dyn Error>> {
        self.nvim.command("vsplit new")?;
        let buf = self.nvim.get_current_buf()?;
        let buf_len = buf.line_count(&mut self.nvim)?;
        buf.set_lines(&mut self.nvim, 0, buf_len, true, lyrics)?;
        let _ = self.nvim.command("setlocal nomodifiable")?;

        Ok(())
    }
}
