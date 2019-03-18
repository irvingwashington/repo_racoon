use std::result::Result;

#[derive(Debug, PartialEq)]
pub struct RepoPath {
    pub organization: String,
    pub repository: String,
    pub path: String,
}

impl RepoPath {
    pub fn parse(path: &str) -> Result<Self, String> {
        let elems: Vec<&str> = path.split('/').collect();
        if elems.len() != 2 {
            return Err(format!("Unknown repo path format: {}", path));
        }

        let organization = String::from(elems[0]);
        let repository = String::from(elems[1]);

        if organization.is_empty() || repository.is_empty() {
            return Err(format!("Incorrect repo: {}", path));
        }

        Ok(RepoPath {
            organization,
            repository,
            path: String::from(path),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_repo_path() {
        let rp = RepoPath {
            organization: String::from("foo"),
            repository: String::from("bar"),
            path: String::from("foo/bar"),
        };
        assert_eq!(RepoPath::parse("foo/bar").unwrap(), rp);
    }

    #[test]
    fn test_incorrect_repo_path() {
        assert_eq!(
            RepoPath::parse("foo"),
            Err(String::from("Unknown repo path format: foo"))
        );
        assert_eq!(
            RepoPath::parse(""),
            Err(String::from("Unknown repo path format: "))
        );
    }

    #[test]
    fn test_many_slash_repo_path() {
        assert_eq!(
            RepoPath::parse("foo/bar/baz"),
            Err(String::from("Unknown repo path format: foo/bar/baz"))
        );
    }

    #[test]
    fn test_short_repo_paths() {
        assert_eq!(RepoPath::parse("/"), Err(String::from("Incorrect repo: /")));
        assert_eq!(
            RepoPath::parse("foo/"),
            Err(String::from("Incorrect repo: foo/"))
        );
        assert_eq!(
            RepoPath::parse("/bar"),
            Err(String::from("Incorrect repo: /bar"))
        );
    }
}
