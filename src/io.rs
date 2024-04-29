use crate::models::Player;
use csv::{Reader, ReaderBuilder};
use std::fs::File;
use std::io::Read;
pub fn read_players(filepath: &str) -> Vec<Player> {
    let file = File::open(filepath).expect("file not found");
    read_players_from_reader(file)
}
//The function “read_players” takes a file path as a string slice and returns a vector of Player objects. 
//Open the file then send it to the function “read_players_from_reader” for further processing
pub fn read_players_from_reader<R: Read>(reader: R) -> Vec<Player> {
    let mut rdr = ReaderBuilder::new().from_reader(reader);
    rdr.deserialize()
        .map(|result| result.expect("error reading line"))
        .collect()
}
//The read_players_from_reader function in Rust is used for converting data read from any input 
//source implementing the Read trait into a structured format, specifically a vector of Player objects.
#[cfg(test)]
mod tests {
    use super::read_players_from_reader;  
    use std::io::Cursor;

    #[test]
    fn test_read_players() {
        let data = "Player,Tm\nLeBron James,LAL\nKevin Durant,BKN";
        let cursor = Cursor::new(data);
        let players = read_players_from_reader(cursor); 

        assert_eq!(players.len(), 2);
        assert_eq!(players[0].Player, "LeBron James");
        assert_eq!(players[0].Tm, "LAL");
        assert_eq!(players[1].Player, "Kevin Durant");
        assert_eq!(players[1].Tm, "BKN");
    }
}


