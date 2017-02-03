use std::io::{Read};
use client::Github;
use hyper::client::Response;
use serde_json::{from_str};
use error::GitError;
use repository::Repo;
use link::Link;
fn get_json(mut res: Response) -> Result<Vec<Repo>, GitError> {
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    let result: Vec<Repo> = from_str(&body)?;
    Ok(result)
}

impl Github {
    pub fn get_org_repos(&self, repo_name: String) -> Result<Vec<Repo>, GitError> {
        let url = format!("https://api.github.com/orgs/{}/repos", repo_name);
        let response = self.make_request(url.as_str())?;
        get_json(response)
    }
}