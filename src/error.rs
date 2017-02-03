use hyper;
use serde_json;
use std::io;
use git2;
#[derive(Debug)]
pub enum GitError {
    Request(hyper::Error),
    JSON(serde_json::Error),
    ReadingBody(io::Error),
    Cloning(git2::Error)
}

impl From<serde_json::Error> for GitError {
     fn from(err: serde_json::Error) -> GitError {
         GitError::JSON(err)
     }
}

impl From<hyper::Error> for GitError {
     fn from(err: hyper::Error) -> GitError {
         GitError::Request(err)
     }
}

impl From<io::Error> for GitError {
     fn from(err: io::Error) -> GitError {
         GitError::ReadingBody(err)
     }
}

impl From<git2::Error> for GitError {
     fn from(err: git2::Error) -> GitError {
         GitError::Cloning(err)
     }
}