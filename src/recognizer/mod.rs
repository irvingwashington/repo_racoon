use crate::app_error::AppError;
use crate::github_repo::GithubRepo;
use std::collections::HashMap;

pub mod ruby;

pub type Properties = HashMap<String, String>;

// {
//   "Airhelp/ah-cockpit":
//   {
//     "languages": [
//       {"name": "Ruby", "version": "2.4.4", "source": ".ruby-version"}
//     ],
//     "tools": [
//       {"name": "docker", "version": "1.2.3"}
//     ]
//   }
// }

pub type RepoProperties = HashMap<String, Vec<Properties>>;

pub fn recognize(repo: &GithubRepo) -> Result<RepoProperties, AppError> {
    let languages = repo.languages()?;

    let mut repo_languages = Vec::new();
    let mut repo_tools = Vec::new();

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

        for lang in result_langs {
            repo_languages.push(lang);
        }

        for tool in result_tools {
            repo_tools.push(tool);
        }
    }
    let mut repo_properties = RepoProperties::new();
    repo_properties.insert("languages".to_string(), repo_languages);
    repo_properties.insert("tools".to_string(), repo_tools);

    Ok(repo_properties)
}
