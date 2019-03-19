use crate::app_error::AppError;
use crate::github_repo::GithubRepo;
use crate::recognizer::{Properties};

pub fn recognize(repo: &GithubRepo, bytes: usize) -> (Vec<Properties>, Vec<Properties>) {
    // Find ruby versions, take versions from those files
    // If no ruby-version file, find Gemfiles, take version from there (if there are)
    // Find gemspecs, take gem names from there
    // Find Rails, Sidekiq, PG/MySQL, Redis in Gemfile.lock
    let ruby_versions = recognize_ruby_versions(repo, bytes);
    let gems = recognize_gems(repo);

    let languages = ruby_versions.unwrap_or_else(|_| Vec::new());
    let tools = gems.unwrap_or_else(|_| Vec::new());
    (languages, tools)
}

fn recognize_ruby_versions(repo: &GithubRepo, _bytes: usize) -> Result<Vec<Properties>, AppError> {
    let ruby_version_files = repo.search_file(".ruby-version".to_string())?;
    let mut rubies = Vec::new();

    for file in ruby_version_files {
        let mut version = repo.file_contents(&file)?;
        version = version.trim_end().to_string();

        let mut props = Properties::with_capacity(3);
        props.insert("source".to_string(), file);
        props.insert("version".to_string(), version.clone());
        props.insert("name".to_string(), "Ruby".to_string());

        rubies.push(props);
    }

    Ok(rubies)
}

fn recognize_gems(repo: &GithubRepo) -> Result<Vec<Properties>, AppError> {
    let gemspecs = repo.search_file("*.gemspec".to_string())?;

    let mut gems = Vec::new();

    for gemspec in gemspecs {
        let mut props = Properties::with_capacity(3);
        let file_with_extension = gemspec.split('/').last().unwrap_or(&gemspec);
        let gem_name = file_with_extension
            .split('.')
            .nth(0)
            .unwrap_or(file_with_extension);

        props.insert("type".to_string(), "Ruby Gem".to_string());
        props.insert("source".to_string(), gemspec.clone());
        props.insert("name".to_string(), gem_name.to_string());
        gems.push(props);
    }

    Ok(gems)
}
