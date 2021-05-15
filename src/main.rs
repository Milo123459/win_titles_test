use window_titles::{Connection, ConnectionTrait};
fn main() {
    let connection = Connection::new().unwrap();
    connection
        .windows()
        .into_iter()
        .filter(|w| w.process().name() == "Spotify.exe")
        .filter(|s| !s.name().unwrap().trim().is_empty())
        .for_each(|x| println!("{}", (x.name().unwrap())))
}
