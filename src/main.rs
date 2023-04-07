use crate::config::config::read_config;
mod config;
mod filter;

fn main() {
    let config = read_config();

    println!("{:?}", config);
}
