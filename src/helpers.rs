//! Helpers for handlebars templates.

use handlebars::{Handlebars, Helper, RenderContext, RenderError};
use serde_json::Value;

const DEFAULT_SEPARATOR: &'static str = ", ";

/// Simple helper to join an array.
///
/// # Parameters
/// - array: The array to join.
///
/// # Example
///
/// ```
/// # extern crate handlebars_iron;
/// # extern crate website;
/// # use handlebars_iron::handlebars;
/// use handlebars::{Handlebars, Template};
///
/// # fn main() {
/// let context = vec![1, 2, 3];
///
/// // Register the template and helper.
/// let mut handlebars = Handlebars::new();
/// handlebars.register_helper("join", Box::new(website::helpers::join));
/// handlebars.register_template_string("template", "{{ join this }}").unwrap();
///
/// let result = handlebars.render("template", &context).unwrap();
/// assert_eq!(result, "1, 2, 3");
/// # }
/// ```
pub fn join(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let array = h
        .param(0)
        .map(|p| p.value())
        .ok_or_else(|| RenderError::new("Missing parameter for `join`"))?;

    let separator = h
        .param(1)
        .map(|p| p.value())
        .and_then(|sep| sep.as_str())
        .unwrap_or(DEFAULT_SEPARATOR);

    let strings = array
        .as_array()
        .ok_or_else(|| RenderError::new("Parameter for `join` must be an array."))?
        .iter()
        .map(|value| match *value {
            Value::String(ref string) => string.to_owned(),
            _ => value.to_string(),
        })
        .collect::<Vec<_>>();
    rc.writer.write_all(strings.join(separator).as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use handlebars::Handlebars;

    #[test]
    fn join() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("join", Box::new(super::join));
        handlebars
            .register_template_string("template", "{{join this}}")
            .unwrap();

        let result = handlebars.render(
            "template",
            &vec!["one".to_owned(), "two".to_owned(), "three".to_owned()],
        );
        assert_eq!(result.unwrap(), "one, two, three");
    }
}
