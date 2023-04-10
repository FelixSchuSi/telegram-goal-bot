use crate::filter::competition::Competition;
use std::env;

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
        let chat_id: i64;
        let chat_id_replies: i64;
        if competition_name == "bundesliga" {
            contents = include_str!("bundesliga.json");
            chat_id = env::var("CHAT_ID_BUNDESLIGA")
                .expect("Environment variable 'CHAT_ID_BUNDESLIGA' missing")
                .parse()
                .unwrap();
            chat_id_replies = env::var("CHAT_ID_BUNDESLIGA_REPLIES")
                .expect("Environment variable 'CHAT_ID_BUNDESLIGA_REPLIES' missing")
                .parse()
                .unwrap();
        } else if competition_name == "champions_league" {
            contents = include_str!("champions_league.json");
            chat_id = env::var("CHAT_ID_CHAMPIONS_LEAGUE")
                .expect("Environment variable 'CHAT_ID_CHAMPIONS_LEAGUE' missing")
                .parse()
                .unwrap();
            chat_id_replies = env::var("CHAT_ID_CHAMPIONS_LEAGUE_REPLIES")
                .expect("Environment variable 'CHAT_ID_CHAMPIONS_LEAGUE_REPLIES' missing")
                .parse()
                .unwrap();
        } else if competition_name == "premier_league" {
            contents = include_str!("premier_league.json");
            chat_id = env::var("CHAT_ID_PREMIER_LEAGUE")
                .expect("Environment variable 'CHAT_ID_PREMIER_LEAGUE' missing")
                .parse()
                .unwrap();
            chat_id_replies = env::var("CHAT_ID_PREMIER_LEAGUE_REPLIES")
                .expect("Environment variable 'CHAT_ID_PREMIER_LEAGUE_REPLIES' missing")
                .parse()
                .unwrap();
        } else if competition_name == "internationals" {
            contents = include_str!("internationals.json");
            chat_id = env::var("CHAT_ID_INTERNATIONALS")
                .expect("Environment variable 'CHAT_ID_INTERNATIONALS' missing")
                .parse()
                .unwrap();
            chat_id_replies = env::var("CHAT_ID_INTERNATIONALS_REPLIES")
                .expect("Environment variable 'CHAT_ID_INTERNATIONALS_REPLIES' missing")
                .parse()
                .unwrap();
        } else {
            panic!("Invalid competition name: {}", competition_name);
        }
        let mut result: Competition = serde_json::from_str(contents)
            .expect(&format!("JSON deserialization of competitions failed"));
        result.chat_id = chat_id;
        result.chat_id_replies = chat_id_replies;
        result
    }
}
