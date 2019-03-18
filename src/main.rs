mod app_error;
mod cli;
mod formatter;
mod github_repo;
mod github_repos;
mod recognizer;
mod repo_path;

extern crate base64;
extern crate clap;
extern crate github_rs;
extern crate serde;
extern crate serde_json;

use github_repo::GithubRepo;
use repo_path::RepoPath;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;
use std::collections::HashMap;

fn repos_to_check(matches: &clap::ArgMatches<'static>, github_token: &str) -> Vec<String> {
    let repos_to_check;

    if let Some(organization) = matches.value_of("organization") {
        let all_repos = github_repos::organization_repos(organization, github_token);

        if all_repos.is_err() {
            eprintln!(
                "Token without permissions ('{}') or incorrect organzation name ('{}')",
                github_token, organization
            );
            std::process::exit(1);
        }
        repos_to_check = all_repos.unwrap();
    } else if let Some(repos) = matches.values_of("repo") {
        repos_to_check = repos.map(|s| s.to_string()).collect();
    } else {
        eprintln!(
            "You need to either provide repos or use the -o option for the whole organization."
        );
        std::process::exit(1);
    };
    repos_to_check
}

fn repo_paths(repos_to_check: Vec<String>) -> Vec<RepoPath> {
    let mut repo_paths = Vec::new();

    for repo in repos_to_check {
        match RepoPath::parse(&repo) {
            Ok(repo_path) => repo_paths.push(repo_path),
            Err(msg) => eprintln!("{}", msg),
        }
    }

    if repo_paths.is_empty() {
        eprintln!("No valid repos, pass at least one repo or use the -o option for the whole organization.");
        std::process::exit(1);
    }
    repo_paths
}

const WORKERS_NUM: usize = 8;

fn main() {
    let matches = cli::match_arguments();
    let github_token = matches.value_of("github_token").unwrap().to_string();

    let pool = ThreadPool::new(WORKERS_NUM);

    let mut result : HashMap<String, Vec<recognizer::NamedProperties>> = HashMap::new();
    let data = Arc::new(Mutex::new(result));

    for repo_path in repo_paths(repos_to_check(&matches, &github_token)) {
        let github_token_copy = github_token.clone();
        let data = Arc::clone(&data);
        pool.execute(move || {
            match GithubRepo::from_repo_path(&repo_path, &github_token_copy) {
                Ok(github_repo) => {
                    if let Ok((languages, tools)) = recognizer::recognize(&github_repo) {
                        let mut data = data.lock().unwrap();
                        eprintln!("Inserting {:?} {:?} to {:?} ({:?})", languages, tools, repo_path.path, data);
                        data.insert(repo_path.path.clone(), vec![languages, tools]);
                    } else {
                        eprintln!("Recognition failed for {:?}", github_repo)
                    }
                },
                Err(error_msg) => eprintln!("Github error for {:?}: {:?}", repo_path, error_msg),
            }
        });
    }
    pool.join();
    println!("Data: {:?}", data);
}
