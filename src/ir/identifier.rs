use heck::ToUpperCamelCase;
use heck::ToSnakeCase;

#[derive(Debug, Clone)]
pub struct Identifier(pub String);

impl Identifier {
    pub fn raw(&self) -> &str {
        &self.0
    }

    pub fn lower(&self) -> String {
        self.0.to_lowercase()
    }

    pub fn upper_camel(&self) -> String {
        self.0.to_upper_camel_case()
    }

    pub fn snake_case(&self) -> String {
        self.0.to_snake_case()
    }
}
