use log::error;
use regex::Regex;
use reqwest::blocking::{self, Client};
use scraper::{Html, Selector};
use std::{collections::HashMap, time::Duration};

use crate::configs::Profile;
use crate::event::Event;

pub struct Captive {
    probe_url: String,
    portal_url: String,
    client: blocking::Client,
    max_concurrent_regex: Regex,
    auth_failed_regex: Regex,
    success_regex: Regex,
}

impl Captive {
    pub fn new(timeout: u64) -> Self {
        Self {
            probe_url: "http://connectivitycheck.gstatic.com/generate_204".to_string(),
            portal_url: String::from(""),
            client: Client::builder()
                .pool_max_idle_per_host(0)
                .timeout(Duration::from_secs(timeout))
                .build()
                .expect("failed to build Client"),
            max_concurrent_regex: Regex::new(
                r"Sorry, user&apos;s concurrent authentication is over limit",
            )
            .unwrap(),
            auth_failed_regex: Regex::new(r"Firewall authentication failed. Please try again.")
                .unwrap(),
            success_regex: Regex::new(r"http://172.16.222.1:1000/keepalive\?").unwrap(),
        }
    }
    pub fn probe(&mut self) -> bool {
        match blocking::get(&self.probe_url) {
            Ok(resp) => {
                if resp.status().is_redirection() {
                    if let Some(loc) = resp.headers().get(reqwest::header::LOCATION) {
                        self.portal_url = loc
                            .to_str()
                            .expect("failed to convert to string")
                            .to_string();
                        return true;
                    } else {
                        return false;
                    }
                }
                let status = resp.status();
                if status.is_success() {
                    let body = resp.text().unwrap_or_default();
                    let re =
                        Regex::new(r#"window\.location=['"](?P<url>https?://[^'"]+)['"]"#).unwrap();
                    match re.captures(&body) {
                        Some(caps) => {
                            self.portal_url = caps["url"].to_string();
                            return true;
                        }
                        None => false,
                    }
                } else {
                    false
                }
            }
            Err(e) => {
                error!("error probing the captive: {}", e);
                false
            }
        }
    }

    pub fn login(&mut self, profile: &Profile) -> Event {
        let resp = self
            .client
            .get(&self.portal_url)
            .send()
            .expect("failed to open the login page");
        if resp.status().is_success() {
            let body = resp.text().unwrap_or_default();
            let login_page_status = self.handle_login_page(&body, profile);
            if login_page_status.0
                && let Some(login_status_page_html) = login_page_status.1
            {
                if let Some(_) = self.success_regex.captures(&login_status_page_html) {
                    return Event::Success;
                } else if let Some(_) = self.auth_failed_regex.captures(&login_status_page_html) {
                    return Event::WrongCreds;
                } else if let Some(_) = self.max_concurrent_regex.captures(&login_status_page_html)
                {
                    return Event::MaxConcurrent;
                }
            }
        }
        Event::Unknown
    }

    fn handle_login_page(&mut self, body: &str, profile: &Profile) -> (bool, Option<String>) {
        let mut submission_data = HashMap::new();
        submission_data.insert(String::from("username"), profile.rollno.clone());
        submission_data.insert(String::from("password"), profile.password.clone());
        let magics = self.extract_magic(body, &mut submission_data);
        println!("{:#?}", magics);
        let domain_re = Regex::new("http?://([^/]+)").unwrap();
        if let Some(protal_domain) = domain_re.captures(&self.portal_url) {
            match self
                .client
                .post(format!(
                    "{}{}",
                    protal_domain[0].to_string(),
                    submission_data.get("submit").unwrap_or(&String::from("/"))
                ))
                .form(&submission_data)
                .send()
            {
                Ok(res) => {
                    if res.status().is_success() {
                        return (true, Some(res.text().unwrap_or_default()));
                    }
                }
                Err(e) => {
                    error!("error attempting login: {}", e);
                    return (false, None);
                }
            }
        }
        (false, None)
    }

    fn extract_magic(&mut self, html: &str, submission_data: &mut HashMap<String, String>) {
        // assumes all the required values are sure to be present in html
        let doc = Html::parse_document(html);
        let form_sel = Selector::parse("form").unwrap();
        let input_sel = Selector::parse("input").unwrap();
        for form in doc.select(&form_sel) {
            let action = form.value().attr("action").unwrap_or("");
            submission_data.insert(String::from("submit"), action.to_string());
            for input in form.select(&input_sel) {
                let name = input.value().attr("name").unwrap_or("");
                let value = input.value().attr("value").unwrap_or("");
                if name == "magic" || name == "4Tredir" {
                    submission_data.insert(name.to_string(), value.to_string());
                }
            }
            break;
        }
    }
}
