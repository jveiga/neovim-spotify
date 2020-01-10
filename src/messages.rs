pub enum Messages {
    CurrentSong,
    PlayPause,
    Play,
    Pause,
    Next,
    Previous,
    Lyrics,
    PlayTrack(String),
    Unknown(String),
}

impl From<String> for Messages {
    // TODO change to return Result
    fn from(event: String) -> Self {
        if event.starts_with("play_track"){
            return Messages::PlayTrack(event[11..].to_string());
        }

        match &event[..] {
            "current_song" => Messages::CurrentSong,
            "play_pause" => Messages::PlayPause,
            "play" => Messages::Play,
            "pause" => Messages::Pause,
            "next" => Messages::Next,
            "previous" => Messages::Previous,
            "lyrics" => Messages::Lyrics,
            _ => Messages::Unknown(event),
        }
    }
}
