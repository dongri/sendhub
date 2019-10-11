// utils
use serde::Serialize;
use tera::Tera;

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn build_template<T: Serialize>(name: &'static str, data: &T) -> Result<String, String> {
    let tt = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            return Err(e.to_string());
        }
    };
    match tt.render(name, &data) {
        Ok(s) => {
            return Ok(s);
        }
        Err(e) => {
            return Err(e.to_string());
        }
    }
}
