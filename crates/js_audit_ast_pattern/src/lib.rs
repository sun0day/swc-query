use regex::Regex;

const BASE_REG:&str = r"([a-zA-Z0-9_-].+|\$\(.+?\))";
const REG_PREFIX:&str = r"$(";
const REG_SUFFIX:&str = r")";

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

  fn unwrap(s: &str) -> String {
    match s.starts_with(REG_PREFIX) {
      true => s.strip_prefix(REG_PREFIX).unwrap().strip_suffix(REG_SUFFIX).unwrap().to_owned(),
      false => s.to_owned()
    }
  }
}

impl Pattern for StaticMemberExpressionPattern {
  fn build(pattern: &str) -> Result<Self, String> {
    let re = Regex::new(&format!(r"^{}\.{}$", BASE_REG, BASE_REG)).unwrap();
    match re.captures(pattern) {
      Some(caps)=> {
        Ok(Self {
          object: Self::unwrap(&caps[1]),
          property: Self::unwrap(&caps[2]),
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

    let pattern = StaticMemberExpressionPattern::build(r"$(.+).$(\()");
    assert_eq!(pattern, Ok(StaticMemberExpressionPattern {
      object: r".+".to_owned(),
      property: r"\(".to_owned()
    }));

    let pattern = StaticMemberExpressionPattern::build("$(.+)");
    assert_eq!(pattern, Err("invalid StaticMemberExpression Pattern $(.+)".to_owned()));
  }
}
