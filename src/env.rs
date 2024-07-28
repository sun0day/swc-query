use std::cmp::min;

pub enum RuleEnv {
  NODE,
  BROWSER,
}

impl RuleEnv {
  pub(crate) fn from(envs: Vec<&str>) -> [Option<RuleEnv>; 2] {
    let mut arr = [None, None];
    for i in 0..min(2, envs.len()) {
      arr[i] = RuleEnv::str2env(envs[i]);
    }

    arr
  }

  pub(crate) fn hit(envs: &[Option<RuleEnv>], estr: Option<&str>) -> bool {
    match estr {
      Some(e) => {
        for env_opt in envs {
          match (env_opt, e) {
            (Some(RuleEnv::NODE), "node") | (Some(RuleEnv::BROWSER), "browser") => {
              return true;
            }
            _ => continue,
          }
        }

        false
      }
      _ => true,
    }
  }

  fn str2env(e: &str) -> Option<RuleEnv> {
    match e.trim() {
      "node" => Some(RuleEnv::NODE),
      "browser" => Some(RuleEnv::BROWSER),
      _ => None,
    }
  }
}
