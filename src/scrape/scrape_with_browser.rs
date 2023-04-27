use std::error::Error;

use headless_chrome::protocol::cdp::Page;
use headless_chrome::{Browser, LaunchOptions};

fn scrape_with_browser(url: &str, selector: &str, attribute: &str) -> Result<(), Box<dyn Error>> {
    let launch_options = LaunchOptions::default_builder().build()?;

    let browser = Browser::new(launch_options)?;

    let tab = browser.new_tab()?;

    tab.navigate_to(url)?;
    let element = tab.wait_for_element(selector)?;
    let function_declaration =
        format!("function() {{ return this.getAttribute('{}') }}", attribute);

    println!("func: {:?}", function_declaration);

    let result = element.call_js_fn("function() { return this.innerText }", vec![], false);

    println!("{:?}", result);
    println!("{:?}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_side_scrape() {
        let result = scrape_with_browser("https://streamff.com/v/qagwUNlcwP", "video", "src");
        println!("{:?}", result);
    }
}
