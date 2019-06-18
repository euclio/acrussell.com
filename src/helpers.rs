//! Helpers for handlebars templates.

use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};
use serde_json::Value;

const DEFAULT_SEPARATOR: &'static str = ", ";

/// Simple helper to join an array.
///
/// # Parameters
/// - array: The array to join.
pub fn join(
    h: &Helper<'_, '_>,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext<'_>,
    out: &mut dyn Output,
) -> HelperResult {
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
    out.write(&strings.join(separator))?;
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
