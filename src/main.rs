extern crate env_logger;
extern crate goji;

use goji::{Credentials, Jira};
use std::env;


fn main() {
    drop(env_logger::init());
    if let(Ok(host), Ok(user), Ok(pass)) = (
        env::var("JIRA_HOST"),
        env::var("JIRA_USER"),
        env::var("JIRA_PASS"),
        )
}
