use headless_chrome::{Browser, LaunchOptions};

use crate::scrape::scrape::ScrapeError;

pub fn scrape_with_browser(
    url: &str,
    selector: &str,
    attribute: &str,
) -> Result<String, ScrapeError> {
    let launch_options = LaunchOptions::default_builder();

    let browser = Browser::new(launch_options.build().map_err(|err| {
        ScrapeError("Error while configuring Browser: ".to_owned() + &*err.to_string())
    })?)
    .map_err(|err| {
        ScrapeError("Error while configuring Browser: ".to_owned() + &*err.to_string())
    })?;
    let tab = browser
        .new_tab()
        .map_err(|err| ScrapeError("Error opening new tab: ".to_owned() + &*err.to_string()))?;
    tab.navigate_to(url).map_err(|err| {
        ScrapeError("Error while navigating to URL: ".to_owned() + &*err.to_string())
    })?;
    println!("{}", tab.get_content().unwrap());
    let element = tab.wait_for_element(selector).map_err(|err| {
        ScrapeError("Could not find Element for given Selector: ".to_owned() + &*err.to_string())
    })?;
    let function_declaration = format!(
        "function temp () {{ return this.getAttribute('{}'); }}",
        attribute
    );
    let remote_object = element.call_js_fn(&function_declaration, vec![], false);

    return remote_object
        .map_err(|err| {
            ScrapeError(
                "Error while extracting Attribute from HTML Element".to_owned() + &*err.to_string(),
            )
        })
        .map(|ok_val| {
            ok_val
                .value
                .expect("Error while extracting remote object")
                .to_string()
                .replace("\"", "")
        });
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
