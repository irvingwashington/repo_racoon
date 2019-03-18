use clap::{App, Arg};

pub fn match_arguments() -> clap::ArgMatches<'static> {
    App::new("RepoRacoon")
        .version("0.1")
        .author("Maciek Dubiński")
        .about("GitHub repository technologies recognizer")
        .arg(
            Arg::with_name("github_token")
                .short("t")
                .value_name("GITHUB_TOKEN")
                .required(true)
                .help("GitHub account token"),
        )
        .arg(
            Arg::with_name("organization")
                .short("o")
                .value_name("ORGANIZATION")
                .help("Checks all repositories of the organization"),
        )
        .arg(
            Arg::with_name("repo")
                .min_values(1)
                .value_name("org/repo")
                .help("Organization/Repo, multiple values accepted"),
        )
        .get_matches()
}
