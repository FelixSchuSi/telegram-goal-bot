use std::fs;

use crate::filter::competition::Competition;

#[derive(Debug)]
pub struct Config {
    pub bundesliga: Competition,
    pub champions_league: Competition,
    pub premier_league: Competition,
    pub internationals: Competition,
}

pub fn read_config() -> Config {
    Config {
        bundesliga: read_competition("bundesliga"),
        champions_league: read_competition("champions_league"),
        premier_league: read_competition("premier_league"),
        internationals: read_competition("internationals"),
    }
}

fn read_competition(competition_name: &str) -> Competition {
    let filename = &format!("{competition_name}.json");

    let file = fs::File::open(format!("src/config/{filename}"))
        .expect(&format!("Opening file {filename} failed"));

    let competition: Competition = serde_json::from_reader(file)
        .expect(&format!("JSON deserialization of file {filename} failed"));
    competition
}
