use std::process;

use serde_json::{from_str, to_string};
use sha2::{Digest, Sha256};

use crate::app::{
  build::{GHRelease, Str},
  ERR, WARN,
};

use super::CLIENT;

pub fn fetch_release(repo: &str, r_id: &str, gh_token: &str) -> (Str, GHRelease) {
  let Ok(resp) = ({
    let mut resp = CLIENT.get(format!(
      "https://api.github.com/repos/{repo}/releases/{r_id}"
    ));

    if gh_token != "" {
      resp = resp.bearer_auth(gh_token);
    } else {
      WARN.println(&"You may set GH_TOKEN environment variable to load private repos");
    }

    resp.send()
  }) else {
    ERR.println(&"Unable to fetch release");
    process::exit(1)
  };

  let Ok(release) = resp.text() else {
    ERR.println(&"Unable to read release");
    process::exit(1);
  };

  let Ok(resp) = from_str::<GHRelease>(&release) else {
    ERR.println(&"Unable to parse release");
    process::exit(1);
  };

  let mut hasher = Sha256::new();

  hasher.update(release.as_bytes());

  let hashed = hasher.finalize();
  let hashed = hashed.to_vec();
  let version = to_string(&hashed).unwrap_or("**UNKNOWN**".to_string());

  (version, resp)
}
