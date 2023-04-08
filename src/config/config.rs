use crate::filter::competition::Competition;

#[derive(Debug)]
pub struct Config {
    pub bundesliga: Competition,
    pub champions_league: Competition,
    pub premier_league: Competition,
    pub internationals: Competition,
}

impl Config {
    pub fn init() -> Config {
        Config {
            bundesliga: Self::read_competition("bundesliga"),
            champions_league: Self::read_competition("champions_league"),
            premier_league: Self::read_competition("premier_league"),
            internationals: Self::read_competition("internationals"),
        }
    }

    fn read_competition(competition_name: &str) -> Competition {
        let contents: &str;
        if competition_name == "bundesliga" {
            contents = include_str!("bundesliga.json");
        } else if competition_name == "champions_league" {
            contents = include_str!("champions_league.json");
        } else if competition_name == "premier_league" {
            contents = include_str!("premier_league.json");
        } else if competition_name == "internationals" {
            contents = include_str!("internationals.json");
        } else {
            panic!("Invalid competition name: {}", competition_name);
        }
        serde_json::from_str(contents)
            .expect(&format!("JSON deserialization of competitions failed"))
    }
}
