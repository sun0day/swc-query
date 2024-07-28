use proc_macro::{TokenStream};
use quote::{quote, IdentFragment};
use syn::{parse, spanned::Spanned, Data, DeriveInput, Ident, LitStr, Meta};

#[proc_macro_derive(JSAuditRule, attributes(audit_rule))]
pub fn js_audit_rule_derive(input: TokenStream) -> TokenStream {
    let ast:DeriveInput = parse(input).unwrap();
    let name = Ident::new(&format!("{}Rule", ast.ident), ast.ident.span());
    // get existing fields
    let fields = match ast.data {
        Data::Struct(data) => {
            Vec::from_iter(data.fields)
        }
        _ => panic!("JSAuditRule can only be derived for structs"),
    };

    // read rule params
    let mut env_params = vec![];
    for attr in ast.attrs {
        if attr.path().is_ident("audit_rule") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("env") {
                        let value = meta.value()?;  
                        let s:LitStr = value.parse()?;  
                        let s = s.value();
                        let s = s.split(",");
                        for env in s {
                            env_params.push(env.to_string());
                        }
                        Ok(())
                } else {
                    Err(meta.error("unsupported rule params"))
                }
            }).unwrap();
        }
    }

    let gen = quote! {
        #[derive(Default)]
        pub struct #name {
            pub(crate) envs: [Option<RuleEnv>;2],
            pub(crate) diagnostics: Vec<OxcDiagnostic>,
            #(#fields),*
        }
        impl #name {
            pub(crate) fn new() -> Self {
                Self {
                    envs: RuleEnv::from(vec![#(#env_params),*]),
                    ..Self::default()
                }
            }
            pub(crate) fn visit(&mut self, program: &Program) {
                self.visit_program(program);
            }
            pub(crate) fn hit_env(&self, e: Option<&str>) -> bool {
                RuleEnv::hit(&self.envs, e)
            }
        }
    };
    gen.into()
}