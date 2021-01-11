//! Configuration

use std::convert::TryFrom;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ScraperRule(pub String);

impl ScraperRule {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

impl TryFrom<&Path> for ScraperRule {
    type Error = anyhow::Error;

    fn try_from(src: &Path) -> anyhow::Result<ScraperRule> {
        if !src.starts_with("/var/usscraper/data") {
            bail!("scraper rule: bad path prefix");
        }
        if !src.ends_with("data.json") {
            bail!("scraper rule: bad suffix");
        }
        let components: Vec<&str> = src
            .components()
            .filter(|c| {
                if let Component::Normal(_) = c {
                    true
                } else {
                    false
                }
            })
            .map(|c| c.as_os_str().to_str().unwrap_or(""))
            .collect();
        if components.len() != 5 {
            bail!("scraper rule: path does not have expected size");
        }
        if components[3] == "" {
            bail!("scraper rule: rule name is not valid utf-8");
        }
        Ok(ScraperRule(components[3].to_string()))
    }
}

impl Into<PathBuf> for &ScraperRule {
    fn into(self) -> PathBuf {
        // 80 = 20(/var/usscraper/data/) + 50(rule name length) + 10(/data.json)
        let mut rule_data_path = PathBuf::with_capacity(80);
        rule_data_path.push("/var/usscraper/data");
        rule_data_path.push(&self.0);
        rule_data_path.push("data.json");
        rule_data_path
    }
}

pub struct ScraperRules(pub Vec<ScraperRule>);

pub struct Config {
    rules: ScraperRules,
}
