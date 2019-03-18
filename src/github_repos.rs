use crate::app_error::AppError;

use github_rs::client::{Executor, Github};
use github_rs::StatusCode;
use serde_json::Value;
use std::collections::hash_map::HashMap;

pub fn organization_repos(organization: &str, github_token: &str) -> Result<Vec<String>, AppError> {
    let client = Github::new(github_token)?;
    let mut path = format!("orgs/{}/repos", organization);
    let mut repos = Vec::new();
    loop {
        let result = client
            .get()
            .custom_endpoint(&path)
            .execute::<Vec<HashMap<String, Value>>>()?;

        let (headers, status, wrapped_json) = result;
        if status != StatusCode::OK {
            return Err(AppError::new(&format!("Non-ok response: {:?}", status)));
        }

        if wrapped_json.is_none() {
            return Err(AppError::new(&format!(
                "Response body issue: {:?}",
                wrapped_json
            )));
        }

        let repo_collection = wrapped_json.unwrap();

        for repo_object in repo_collection {
            if let Some(repo_name_value) = repo_object.get("full_name") {
                if let Value::String(repo_name) = repo_name_value {
                    repos.push(repo_name.clone());
                }
            }
        }

        let link_header = headers.get("link");
        if link_header.is_none() {
            break;
        }
        let next_link = link_header
            .unwrap()
            .to_str()
            .unwrap_or("")
            .split(',')
            .find(|el| el.contains("rel=\"next\""));
        if next_link.is_none() {
            break;
        }
        if let Some(url) = next_link.unwrap().split(">;").nth(0) {
            path = url.trim().replace("<https://api.github.com/", "");
        } else {
            break;
        }
    }
    Ok(repos)
}
