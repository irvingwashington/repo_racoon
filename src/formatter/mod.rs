mod json;

use crate::repos_info::ReposInfo;

pub enum Formats {
    JSON
}

pub fn output_formatted(repos_info: &ReposInfo, format: Formats) {
    match format {
        Formats::JSON => {
            if let Ok(json) = serde_json::to_string(&repos_info) {
                println!("{}", json);
            }
        }
    }
}