#[macro_use] extern crate hyper;
extern crate hyper_native_tls;
extern crate serde_json;
extern crate git2;
extern crate regex;
extern crate rayon;
#[macro_use] extern crate serde_derive;

mod link;
mod client;
mod organisations;
mod error;
mod repository;
mod get_info;
mod keys;

use rayon::prelude::*;

fn main() {
   let github = client::Github::new().add_auth(keys::get_github_key());
   let repos = github.get_org_repos("servo".to_string()).unwrap();
   let result: Vec<Result<(), error::GitError>> = repos.into_par_iter().map(get_info::clone).collect();
   println!("END {:?}",result);
}
