use anyhow::Result;
use headless_chrome::protocol::cdp::Page;
use headless_chrome::protocol::cdp::Runtime;
use headless_chrome::{Browser, LaunchOptions};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

pub fn scrape_with_browser(
    url: &str,
    selector: &str,
    attribute: &str,
) -> Result<String, Box<dyn Error>> {
    let launch_options = LaunchOptions::default_builder();
    let browser = Browser::new(launch_options.build()?)?;
    let tab = browser.new_tab()?;
    tab.navigate_to(url)?;
    let element = tab.wait_for_element(selector)?;
    let function_declaration = format!(
        "function temp () {{ return this.getAttribute('{}'); }}",
        attribute
    );
    let remote_object = element.call_js_fn(&function_declaration, vec![], false)?;
    match remote_object.value {
        Some(returned_string) => {
            return Ok(returned_string.to_string().replace("\"", ""));
        }
        _ => unreachable!(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_side_scrape() {
        let result = scrape_with_browser("https://streamff.com/v/qagwUNlcwP", "video", "src");
        assert_eq!(&result.unwrap(), "https://files.catbox.moe/3cdo9q.mp4");
    }
}
