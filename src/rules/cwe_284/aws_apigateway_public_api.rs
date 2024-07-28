use std::fmt::Debug;

use crate::env::RuleEnv;
use js_audit_rule::JSAuditRule;
use oxc_allocator::Box;
use oxc_ast::{
  ast::{
    CallExpression, Expression, ExpressionStatement, IdentifierName, ImportDeclaration,
    ImportSpecifier, ObjectExpression, ObjectProperty, ObjectPropertyKind, Program,
    PropertyKey::StaticIdentifier, StaticMemberExpression, StringLiteral,
  },
  visit::walk::walk_import_declaration,
  Visit,
};
use oxc_diagnostics::OxcDiagnostic;

const AWS_APIGATEWAY: &str = "aws_apigateway";
const AWS_APIGATEWAY_V2: &str = "aws_apigatewayv2";
const AWS_APIGATEWAY_METHOD: &str = "CfnMethod";
const AWS_APIGATEWAY_V2_METHOD: &str = "CfnRoute";
const AWS_APIGATEWAY_ARG: &str = "authorizationType";
const AWS_APIGATEWAY_NO_AUTH: &str = "NONE";

#[derive(JSAuditRule)]
#[audit_rule(env = "node")]
struct AwsAPIGatewayPublicAPI {
  is_aws_lib: bool,
  //   is_aws_call: bool,
  is_aws_set_authorization: bool,
  aws_specifier_map: Vec<(String, String)>,
}

impl AwsAPIGatewayPublicAPIRule {
  fn match_specifier(name: &str) -> bool {
    name == AWS_APIGATEWAY || name == AWS_APIGATEWAY_V2
  }

  fn match_obj_prop(obj_name: &str, prop_name: &str) -> bool {
    match (obj_name, prop_name) {
      (AWS_APIGATEWAY, AWS_APIGATEWAY_METHOD) | (AWS_APIGATEWAY_V2, AWS_APIGATEWAY_V2_METHOD) => {
        true
      }
      _ => false,
    }
  }

  // judge whether is calling aws func
  fn is_aws_call(&self, mem_expr: &Box<StaticMemberExpression>) -> bool {
    let obj_name = match &mem_expr.object {
      Expression::Identifier(id) => {
        let name = id.name.to_string();
        let mut obj_name = None;
        for specifiers in &self.aws_specifier_map {
          if specifiers.0 == name {
            obj_name = Some(specifiers.1.clone());
            break;
          }
        }
        obj_name
      }
      _ => None,
    };

    if let Some(obj_name) = obj_name {
      let prop_name = mem_expr.property.name.to_string();
      return AwsAPIGatewayPublicAPIRule::match_obj_prop(&obj_name, &prop_name);
    }

    false
  }

  // authorizationType invalid
  fn get_invalid_authorization_type(&mut self, obj_expr: &Box<ObjectExpression>) {
    for prop_kind in &obj_expr.properties {
      if let ObjectPropertyKind::ObjectProperty(prop) = prop_kind {
        if let StaticIdentifier(id) = &prop.key {
          let name = id.name.to_string();
          match (name.as_str(), &prop.value) {
            (AWS_APIGATEWAY_ARG, Expression::StringLiteral(lit)) => {
              if lit.value.as_str() == AWS_APIGATEWAY_NO_AUTH {
                self
                  .diagnostics
                  .push(OxcDiagnostic::error(lit.value.to_string()));
              }
            }
            _ => (),
          }
        }
      }
    }
  }
}

impl<'a> Visit<'a> for AwsAPIGatewayPublicAPIRule {
  fn visit_import_declaration(&mut self, decl: &ImportDeclaration<'a>) {
    if decl.source.value == "aws-cdk-lib" {
      self.is_aws_lib = true;
      walk_import_declaration(self, decl);
    }
  }

  fn visit_import_specifier(&mut self, it: &ImportSpecifier<'a>) {
    let imported = it.imported.name().to_string();
    if Self::match_specifier(&imported) {
      self
        .aws_specifier_map
        .push((it.local.name.to_string(), imported));
    }
  }

  fn visit_expression(&mut self, expr: &Expression<'a>) {
    self.is_aws_set_authorization = false;
    match expr {
      Expression::NewExpression(new_expr) => {
        let is_aws_call = match &new_expr.callee {
          Expression::StaticMemberExpression(mem_expr) => self.is_aws_call(mem_expr),
          _ => false,
        };
        if is_aws_call && new_expr.arguments.len() >= 3 {
          match new_expr.arguments[2].to_expression() {
            Expression::ObjectExpression(obj_expr) => {
              self.get_invalid_authorization_type(obj_expr);
            }
            _ => {}
          }
        }
      }
      _ => {}
    }
  }
}
