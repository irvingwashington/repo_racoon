use crate::app_error::AppError;
use crate::repo_path::RepoPath;

use github_rs::client::{Executor, Github};
use github_rs::{errors, HeaderMap, StatusCode};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

pub struct GithubRepo<'a> {
    pub path: &'a RepoPath,
    client: Github,
}

impl<'a> fmt::Debug for GithubRepo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GithubRepo {{path: {:?}}}", self.path)
    }
}

impl<'a> GithubRepo<'a> {
    pub fn from_repo_path(repo_path: &'a RepoPath, github_token: &str) -> Result<Self, AppError> {
        let client = Github::new(github_token)?;

        let repo_info_result = client
            .get()
            .repos()
            .owner(&repo_path.organization)
            .repo(&repo_path.repository)
            .execute::<Value>();

        Self::unwrap_result(repo_info_result)?;

        Ok(GithubRepo {
            path: repo_path,
            client,
        })
    }

    fn unwrap_result<T>(
        result: Result<(HeaderMap, StatusCode, Option<T>), errors::Error>,
    ) -> Result<T, AppError> {
        let (_headers, status, wrapped_json) = result?;

        if status != StatusCode::OK {
            return Err(AppError::new(&format!("Non-ok response: {:?}", status)));
        }

        wrapped_json.ok_or_else(|| AppError::new("Empty or incorrect json"))
    }

    pub fn files(&self, path: String) -> Result<Vec<String>, AppError> {
        let files_result = self
            .client
            .get()
            .repos()
            .owner(&self.path.organization)
            .repo(&self.path.repository)
            .contents()
            .path(&path)
            .execute::<Vec<HashMap<String, Value>>>();

        let files = Self::unwrap_result(files_result)?;
        let mut files_list: Vec<String> = Vec::new();

        for file_obj in files {
            if let Some(file_name) = file_obj.get("path") {
                if let Value::String(str_file_name) = file_name {
                    files_list.push(str_file_name.to_string())
                }
            }
        }

        Ok(files_list)
    }

    pub fn search_file(&self, file_name: String) -> Result<Vec<String>, AppError> {
        let search_endpoint = format!(
            "search/code?q={}+in:path+repo:{}%2F{}", // TODO: urlencode file_name
            file_name, self.path.organization, self.path.repository
        );

        let results_result = self
            .client
            .get()
            .custom_endpoint(&search_endpoint)
            .execute::<HashMap<String, Value>>();
        let results = Self::unwrap_result(results_result)?;

        let items = results.get("items");

        if items.is_none() {
            return Err(AppError::new("No search result field `items`"));
        }
        let mut files = Vec::new();

        if let Value::Array(items_vec) = items.unwrap() {
            for item in items_vec {
                if let Value::Object(item_hash) = item {
                    let path_entry = item_hash.get("path");
                    if path_entry.is_none() {
                        continue;
                    }

                    if let Value::String(full_path) = path_entry.unwrap() {
                        files.push(full_path.to_string());
                    }
                }
            }
        }
        Ok(files)
    }

    pub fn languages(&self) -> Result<HashMap<String, usize>, AppError> {
        let languages_result = self
            .client
            .get()
            .repos()
            .owner(&self.path.organization)
            .repo(&self.path.repository)
            .languages()
            .execute::<HashMap<String, usize>>();

        Self::unwrap_result(languages_result)
    }

    pub fn file_contents(&self, path: &str) -> Result<String, AppError> {
        let contents_result = self
            .client
            .get()
            .repos()
            .owner(&self.path.organization)
            .repo(&self.path.repository)
            .contents()
            .path(&path)
            .execute::<HashMap<String, Value>>();

        let contents = Self::unwrap_result(contents_result)?;
        let content_field_value = contents.get("content");

        if content_field_value.is_none() {
            return Err(AppError::new(
                "File contents call response is missing the `content` field",
            ));
        }

        let base64_content: String;

        match content_field_value.unwrap() {
            Value::String(str) => base64_content = str.replace("\n", "").to_string(),
            _ => return Err(AppError::new("Unsupported `content` field type")),
        }
        let bytes = base64::decode(&base64_content)?;
        let contents_str = std::str::from_utf8(&bytes)?;

        Ok(contents_str.to_string())
    }
}
