//! Helpers for handlebars templates.

use handlebars::{Handlebars, RenderError, RenderContext, Helper, Context};
use serde_json::Value;

const DEFAULT_SEPARATOR: &'static str = ", ";

/// Simple helper to join an array.
///
/// # Parameters
/// - array: The array to join.
///
/// # Example
/// Context:
/// `[1, 2, 3]`
///
/// Template:
/// ```
/// {{join this}}
/// ```
///
/// Result:
/// `"1, 2, 3"`
pub fn join(_: &Context,
            h: &Helper,
            _: &Handlebars,
            rc: &mut RenderContext)
            -> Result<(), RenderError> {
    let array = try!(h.param(0)
        .map(|p| p.value())
        .ok_or_else(|| RenderError::new("Missing parameter for `join`")));

    let separator =
        h.param(1).map(|p| p.value()).and_then(|sep| sep.as_str()).unwrap_or(DEFAULT_SEPARATOR);

    let strings = try!(array.as_array()
            .ok_or_else(|| RenderError::new("Parameter for `join` must be an array.")))
        .iter()
        .map(|value| {
            match *value {
                Value::String(ref string) => string.to_owned(),
                _ => panic!("unexpected value"),
            }
        })
        .collect::<Vec<_>>();
    try!(rc.writer.write(strings.join(separator).as_bytes()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use handlebars::Handlebars;
    use handlebars::Template;

    #[test]
    fn join() {
        let template = Template::compile("{{join this}}").unwrap();

        let mut handlebars = Handlebars::new();
        handlebars.register_helper("join", Box::new(super::join));
        handlebars.register_template("template", template);

        let result =
            handlebars.render("template",
                              &vec!["one".to_owned(), "two".to_owned(), "three".to_owned()]);
        assert_eq!(result.unwrap(), "one, two, three");
    }
}
