use regex::Regex;

const BASE_REG:&str = r"([a-zA-Z0-9_-].+|\$\(.+?\))";

#[derive(Debug, PartialEq)]
pub struct StaticMemberExpressionPattern {
  object: String,
  property: String,
}

pub struct NewExpressionPattern {
  callee: StaticMemberExpressionPattern,
  arguments: Vec<ExpressionPattern>,
}

pub enum ExpressionPattern {
  Any,
  Identifier(String),
  StaticMemberExpression(StaticMemberExpressionPattern),
  NewExpression(NewExpressionPattern),
}

pub trait Pattern {
  fn build(pattern: &str) -> Result<Self, String>
  where
    Self: Sized;
}

impl Pattern for StaticMemberExpressionPattern {
  fn build(pattern: &str) -> Result<Self, String> {
    let re = Regex::new(&format!(r"^{}\.{}$", BASE_REG, BASE_REG)).unwrap();
    match re.captures(pattern) {
      Some(caps)=> {
        Ok(Self {
          object: caps[1].to_string(),
          property: caps[2].to_string()
        })
      },
      None => {
        Err(format!("invalid StaticMemberExpression Pattern {}", pattern))
      }
    }
  }
}

// impl<'a> Pattern for NewExpressionPattern<'a> {
//   fn build(pattern: &str) -> Result<Self, String> {
//     let pat = pattern.trim();

//     if !pat.starts_with("new") {
//       return Err(format!("invalid NewExpression pattern {}", pattern));
//     }

//     let static_member_expr_pat =
//       StaticMemberExpressionPattern::build(pat.strip_prefix("new").unwrap())?;

//     Ok(())
//   }
// }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn build_static_member_expression_pattern() {
    let pattern = StaticMemberExpressionPattern::build("oO1.pP1");
    assert_eq!(pattern, Ok(StaticMemberExpressionPattern {
      object: "oO1".to_owned(),
      property: "pP1".to_owned()
    }));

    let pattern = StaticMemberExpressionPattern::build("$(.+).$(.+)");
    assert_eq!(pattern, Ok(StaticMemberExpressionPattern {
      object: "$(.+)".to_owned(),
      property: "$(.+)".to_owned()
    }));

    let pattern = StaticMemberExpressionPattern::build("$(.+)");
    assert_eq!(pattern, Err("invalid StaticMemberExpression Pattern $(.+)".to_owned()));
  }
}
