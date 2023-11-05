use crate::filter::competition::{Competition, CompetitionName};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub bundesliga: Competition,
    pub bundesliga_2: Competition,
    pub champions_league: Competition,
    pub premier_league: Competition,
    pub internationals: Competition,
}

impl IntoIterator for Config {
    type Item = Competition;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            self.bundesliga,
            self.bundesliga_2,
            self.champions_league,
            self.premier_league,
            self.internationals,
        ]
        .into_iter()
    }
}

impl Config {
    pub fn init() -> Config {
        Config {
            bundesliga: Self::read_competition(CompetitionName::Bundesliga),
            bundesliga_2: Self::read_competition(CompetitionName::Bundesliga2),
            champions_league: Self::read_competition(CompetitionName::ChampionsLeague),
            premier_league: Self::read_competition(CompetitionName::PremierLeague),
            internationals: Self::read_competition(CompetitionName::Internationals),
        }
    }

    fn read_competition(competition_name: CompetitionName) -> Competition {
        let contents: &str;
        let chat_id: i64;
        let chat_id_replies: i64;
        match competition_name {
            CompetitionName::Bundesliga => {
                contents = include_str!("bundesliga.json");
                chat_id = env::var("CHAT_ID_BUNDESLIGA")
                    .expect("Environment variable 'CHAT_ID_BUNDESLIGA' missing")
                    .parse()
                    .unwrap();
                chat_id_replies = env::var("CHAT_ID_BUNDESLIGA_REPLIES")
                    .expect("Environment variable 'CHAT_ID_BUNDESLIGA_REPLIES' missing")
                    .parse()
                    .unwrap();
            }
            CompetitionName::Bundesliga2 => {
                contents = include_str!("bundesliga_2.json");
                chat_id = env::var("CHAT_ID_BUNDESLIGA_2")
                    .expect("Environment variable 'CHAT_ID_BUNDESLIGA_2' missing")
                    .parse()
                    .unwrap();
                chat_id_replies = env::var("CHAT_ID_BUNDESLIGA_2_REPLIES")
                    .expect("Environment variable 'CHAT_ID_BUNDESLIGA_2_REPLIES' missing")
                    .parse()
                    .unwrap();
            }
            CompetitionName::PremierLeague => {
                contents = include_str!("premier_league.json");
                chat_id = env::var("CHAT_ID_PREMIER_LEAGUE")
                    .expect("Environment variable 'CHAT_ID_PREMIER_LEAGUE' missing")
                    .parse()
                    .unwrap();
                chat_id_replies = env::var("CHAT_ID_PREMIER_LEAGUE_REPLIES")
                    .expect("Environment variable 'CHAT_ID_PREMIER_LEAGUE_REPLIES' missing")
                    .parse()
                    .unwrap();
            }
            CompetitionName::ChampionsLeague => {
                contents = include_str!("champions_league.json");
                chat_id = env::var("CHAT_ID_CHAMPIONS_LEAGUE")
                    .expect("Environment variable 'CHAT_ID_CHAMPIONS_LEAGUE' missing")
                    .parse()
                    .unwrap();
                chat_id_replies = env::var("CHAT_ID_CHAMPIONS_LEAGUE_REPLIES")
                    .expect("Environment variable 'CHAT_ID_CHAMPIONS_LEAGUE_REPLIES' missing")
                    .parse()
                    .unwrap();
            }
            CompetitionName::Internationals => {
                contents = include_str!("internationals.json");
                chat_id = env::var("CHAT_ID_INTERNATIONALS")
                    .expect("Environment variable 'CHAT_ID_INTERNATIONALS' missing")
                    .parse()
                    .unwrap();
                chat_id_replies = env::var("CHAT_ID_INTERNATIONALS_REPLIES")
                    .expect("Environment variable 'CHAT_ID_INTERNATIONALS_REPLIES' missing")
                    .parse()
                    .unwrap();
            }
        }
        let mut result: Competition = serde_json::from_str(contents)
            .expect(&format!("JSON deserialization of competitions failed"));
        result.chat_id = chat_id;
        result.chat_id_replies = chat_id_replies;
        result
    }
}
