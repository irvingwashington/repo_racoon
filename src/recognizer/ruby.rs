use crate::app_error::AppError;
use crate::github_repo::GithubRepo;
use crate::recognizer::{NamedProperties, Properties};

pub fn recognize(repo: &GithubRepo, bytes: usize) -> (NamedProperties, NamedProperties) {
    // Find ruby versions, take versions from those files
    // If no ruby-version file, find Gemfiles, take version from there (if there are)
    // Find gemspecs, take gem names from there
    // Find Rails, Sidekiq, PG/MySQL, Redis in Gemfile.lock
    let ruby_versions = recognize_ruby_versions(repo, bytes);
    let gems = recognize_gems(repo);

    let languages = ruby_versions.unwrap_or_else(|_| NamedProperties::new());
    let tools = gems.unwrap_or_else(|_| NamedProperties::new());
    (languages, tools)
}

fn recognize_ruby_versions(repo: &GithubRepo, _bytes: usize) -> Result<NamedProperties, AppError> {
    let ruby_version_files = repo.search_file(".ruby-version".to_string())?;
    let mut rubies = NamedProperties::with_capacity(ruby_version_files.len());

    for file in ruby_version_files {
        let mut version = repo.file_contents(&file)?;
        version = version.trim_end().to_string();

        let mut props = Properties::with_capacity(2);

        props.insert("Version".to_string(), version.clone());
        props.insert("Source".to_string(), file);

        rubies.insert(format!("Ruby {}", version), props);
    }

    Ok(rubies)
}

fn recognize_gems(repo: &GithubRepo) -> Result<NamedProperties, AppError> {
    let gemspecs = repo.search_file("*.gemspec".to_string())?;

    let mut gems = NamedProperties::with_capacity(gemspecs.len());

    for gemspec in gemspecs {
        let mut props = Properties::with_capacity(2);
        props.insert("Type".to_string(), "Ruby Gem".to_string());
        props.insert("Source".to_string(), gemspec.clone());
        let file_with_extension = gemspec.split('/').last().unwrap_or(&gemspec);
        let gem_name = file_with_extension
            .split('.')
            .nth(0)
            .unwrap_or(file_with_extension);
        gems.insert(gem_name.to_string(), props);
    }

    Ok(gems)
}
