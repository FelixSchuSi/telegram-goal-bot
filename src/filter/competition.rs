use std::ops::Index;

use serde::Deserialize;

use crate::config::config::{read_config, Config};

#[derive(Debug, Deserialize)]
pub struct Competition {
    #[allow(dead_code)]
    teams: Vec<Team>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Team {
    min_matches_needed: u32,
    aliases: Vec<String>,
}

pub trait IsValidCompetition {
    fn is_valid_post_title_for_competition(&self, potential_team_name: &str) -> bool;
}

impl IsValidCompetition for Competition {
    fn is_valid_post_title_for_competition(&self, potential_team_name: &str) -> bool {
        let items = potential_team_name.split("-").collect::<Vec<&str>>();
        if items.len() < 2 {
            return false;
        }

        let team1 = items.index(0).trim();
        let team2 = items.index(1).trim();

        self.teams.iter().any(|t| t.is_valid_team(team1))
            && self.teams.iter().any(|t| t.is_valid_team(team2))
    }
}

pub trait IsValidTeam {
    fn is_valid_team(&self, potential_team_name: &str) -> bool;
}

impl IsValidTeam for Team {
    fn is_valid_team(&self, potential_team_name: &str) -> bool {
        let words: Vec<String> = potential_team_name
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .collect();

        self.aliases.iter().filter(|&a| words.contains(&a)).count()
            >= self.min_matches_needed.try_into().unwrap()
    }
}

#[test]
fn is_valid_post_test() {
    let config = read_config();
    let competitions = vec![
        config.bundesliga,
        config.champions_league,
        config.premier_league,
        config.internationals,
    ];
    for competition in competitions {
        let positive_cases = generate_positive_test_cases(&competition);
        println!("{:?}", positive_cases);
        positive_cases.iter().for_each(|c| {
            assert!(
                competition.is_valid_post_title_for_competition(c),
                "\n\n Post falsely NOT identified: {c}\n\n",
            );
        });
    }
}

#[test]
fn test123() {
    let champions_league = read_config().champions_league;
    let title = "Benfica [1] - 0 Porto - Diogo Costa 10' OG";
    assert!(
        champions_league.is_valid_post_title_for_competition(title),
        "\n\n cl post falsely identified: {title}\n\n",
    );
}

#[test]
fn is_not_valid_competition_test() {
    let bundesliga = read_config().bundesliga;

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
