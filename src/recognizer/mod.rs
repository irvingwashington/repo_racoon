use crate::app_error::AppError;
use crate::github_repo::GithubRepo;
use std::collections::HashMap;

pub mod ruby;

pub type Properties = HashMap<String, String>;
pub type NamedProperties = HashMap<String, Properties>;

pub fn recognize(repo: &GithubRepo) -> Result<(NamedProperties, NamedProperties), AppError> {
    let mut recognized_languages = NamedProperties::new();
    let mut recognized_tools = NamedProperties::new();

    let languages = repo.languages()?;

    for (language, bytes) in languages {
        let result = match language.as_ref() {
            "Ruby" => Some(ruby::recognize(repo, bytes)),
            "JavaScript" => None,
            _ => None,
        };

        if result.is_none() {
            continue;
        }

        let (result_langs, result_tools) = result.unwrap();

        for (lang_name, properties) in result_langs {
            recognized_languages.insert(lang_name, properties);
        }

        for (tool, properties) in result_tools {
            recognized_tools.insert(tool, properties);
        }
    }
    Ok((recognized_languages, recognized_tools))
}
