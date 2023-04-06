use std::str::FromStr;

use crate::config::config::read_config;
use crate::filter::competition::IsValidCompetition;
use crate::filter::videohost::VideoHost;
mod config;
mod filter;

fn main() {
    let f = VideoHost::from_str("streamwo").unwrap();
    assert_eq!(f, VideoHost::Streamwo);
    println!("{}", f);

    let config = read_config();
    println!(
        "{:?}",
        config
            .bundesliga
            .is_valid_competition("Bayern Munich 4 - [2] Borussia Dortmund - Donyell Malen 90'")
    );
}
