use std::ops::Index;

use serde::Deserialize;
use teloxide::types::ChatId;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum CompetitionName {
    #[serde(rename = "bundesliga")]
    Bundesliga,
    #[serde(rename = "bundesliga_2")]
    Bundesliga2,
    #[serde(rename = "premier_league")]
    PremierLeague,
    #[serde(rename = "champions_league")]
    ChampionsLeague,
    #[serde(rename = "internationals")]
    Internationals,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Competition {
    teams: Vec<Team>,
    pub name: CompetitionName,
    #[serde(skip)]
    pub chat_id: i64,
    #[serde(skip)]
    pub chat_id_replies: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Team {
    min_matches_needed: u32,
    aliases: Vec<String>,
}

impl Competition {
    pub fn is_valid_post_title_for_competition(&self, potential_team_name: &str) -> bool {
        let items = potential_team_name.split("-").collect::<Vec<&str>>();
        if items.len() < 2 {
            return false;
        }

        let team1 = items.index(0).trim();
        let team2 = items.index(1).trim();

        self.teams.iter().any(|t| t.is_valid_team(team1))
            && self.teams.iter().any(|t| t.is_valid_team(team2))
    }

    pub fn get_chat_id(&self) -> ChatId {
        ChatId(-1000000000000 + self.chat_id)
    }

    pub fn get_chat_id_replies(&self) -> ChatId {
        ChatId(-1000000000000 + self.chat_id_replies)
    }
}

impl Team {
    fn is_valid_team(&self, potential_team_name: &str) -> bool {
        let words: Vec<String> = potential_team_name
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .collect();

        self.aliases.iter().filter(|&a| words.contains(&a)).count()
            >= self.min_matches_needed.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::config::config::Config;
    use std::env;

    use super::*;

    #[test]
    fn is_valid_post_test() {
        mock_env_vars();
        let config = Config::init();
        let competitions = vec![
            config.bundesliga,
            config.bundesliga_2,
            config.champions_league,
            config.premier_league,
            config.internationals,
        ];
        for competition in competitions {
            let positive_cases = generate_positive_test_cases(&competition);
            positive_cases.iter().for_each(|c| {
                assert!(
                    competition.is_valid_post_title_for_competition(c),
                    "\n\n Post falsely NOT identified: {c}\n\n",
                );
            });
        }
    }

    #[test]
    fn test_cl() {
        mock_env_vars();
        let champions_league = Config::init().champions_league;
        let title = "Real Madrid [1] - 0 Manchester City - Diogo Costa 10' OG";
        assert!(
            champions_league.is_valid_post_title_for_competition(title),
            "\n\n cl post falsely identified: {title}\n\n",
        );
    }

    #[test]
    fn test_bundesliga() {
        mock_env_vars();
        let bundesliga = Config::init().bundesliga;
        let title = "Hoffenheim 0 - [1] Borussia Dortmund - Niclas Fullkrug 18'";
        assert!(
            bundesliga.is_valid_post_title_for_competition(title),
            "\n\n bundesliga post falsely not identified: {title}\n\n",
        );
    }

    #[test]
    fn test_bundesliga_2() {
        mock_env_vars();
        let bundesliga_2 = Config::init().bundesliga_2;
        let title = "Hannover [1]-0 Eintracht Braunschweig - Fabian Kunze 12'";
        assert!(
            bundesliga_2.is_valid_post_title_for_competition(title),
            "\n\n bundesliga 2 post falsely not identified: {title}\n\n",
        );
    }

    #[test]
    fn test_bundesliga_heidenheim() {
        mock_env_vars();
        let bundesliga = Config::init().bundesliga;
        let title = "Heidenheim [1]-0 Union Berlin - Jan-Niklas Beste 60' (Great Freekick)";
        assert!(
            bundesliga.is_valid_post_title_for_competition(title),
            "bundesliga post falsely not identified: {title}\n",
        );
    }

    #[test]
    fn is_not_valid_competition_test() {
        mock_env_vars();
        let bundesliga = Config::init().bundesliga;

        let positive_cases = vec![
            "Bayyern Munnich 4 - [2] Borussia Dortmund - Donyell Malen 90'",
            "Bayern München 1-[1] Freiiburg - Nicolas Höfler 27' (Great Goal)",
            "RB Leiipzig 0-[3] Mainz - Domink Kohr 67'",
        ];

        positive_cases.iter().for_each(|c| {
            assert!(
                !bundesliga.is_valid_post_title_for_competition(c),
                "\n\n Bundesliga post falsely identified: {c}\n\n",
            );
        });
    }

    fn generate_positive_test_cases(competition: &Competition) -> Vec<String> {
        competition
            .teams
            .iter()
            .flat_map(|t| {
                t.aliases.iter().map(|a| -> String {
                    if t.min_matches_needed > 1 {
                        let n = t.aliases.len();
                        let left = format!("{} {}", t.aliases[0 % n], t.aliases[1 % n]);
                        let right = format!("{} {}", t.aliases[2 % n], t.aliases[3 % n]);
                        return format!(
                            "{} 4 - [2] {} - Donyell Malen 90'",
                            left.to_uppercase(),
                            right
                        );
                    }
                    format!("{} 4 - [2] {} - Donyell Malen 90'", a.to_uppercase(), a)
                })
            })
            .collect()
    }

    fn mock_env_vars() {
        env::set_var("CHAT_ID_BUNDESLIGA", "123");
        env::set_var("CHAT_ID_BUNDESLIGA_REPLIES", "123");
        env::set_var("CHAT_ID_BUNDESLIGA_2", "123");
        env::set_var("CHAT_ID_BUNDESLIGA_2_REPLIES", "123");
        env::set_var("CHAT_ID_CHAMPIONS_LEAGUE", "123");
        env::set_var("CHAT_ID_CHAMPIONS_LEAGUE_REPLIES", "123");
        env::set_var("CHAT_ID_PREMIER_LEAGUE", "123");
        env::set_var("CHAT_ID_PREMIER_LEAGUE_REPLIES", "123");
        env::set_var("CHAT_ID_INTERNATIONALS", "123");
        env::set_var("CHAT_ID_INTERNATIONALS_REPLIES", "123");
    }
}
