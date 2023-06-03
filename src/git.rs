use std::env;

use octocrab::Octocrab;
use serde::Deserialize;

use crate::error::Error;

#[derive(Deserialize, Debug)]
pub(crate) struct GHFile {
    name: String,
    patch: String,
    sha: String,
}

#[derive(Debug, Clone)]
pub struct GHAction {}

impl GHAction {
    fn commits(&self, event: &str) -> octocrab::Result<()> {
        let mut found: Vec<String> = Vec::new();

        let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
        let octocrab = Octocrab::builder().personal_token(token).build()?;

        let repo = env::var("GITHUB_REPOSITORY");

        match event {
            "pull_request" => {
                if repo.is_ok() {
                    println!("{:?}", env::var("GITHUB_CONTEXT"));
                }
            }
            "push" => {}
            _ => {
                println!("Unknown event type: {}", event);
            }
        }

        Ok(())
    }
}
