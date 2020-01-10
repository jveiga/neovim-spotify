use soup::prelude::*;

pub async fn find_lyrics(artist: &str, song: &str) -> Option<String> {
    let artist = artist.split_whitespace().collect::<Vec<_>>().join("-");
    let song = song.split_whitespace().collect::<Vec<_>>().join("-");
    let ident = format!("{}-{}-lyrics", artist, song);
    let url = format!("https://www.genius.com/{}", ident).replace("'", "");

    let resp = reqwest::get(&url).unwrap();
    let soup = Soup::from_reader(resp).unwrap();
    let lyrics = soup.tag("p").find().unwrap().text();

    if lyrics.trim() == "Sorry, we didn't mean for that to happen!" {
        return None;
    }

    let lyrics = format!("{} - {}\n {}", artist, song, lyrics);

    Some(lyrics.to_owned())
}
