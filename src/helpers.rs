//! Helpers for handlebars templates.

use handlebars::{Handlebars, RenderError, RenderContext, Helper, Context};
use serde_json::Value;

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
pub fn join(c: &Context,
            h: &Helper,
            _: &Handlebars,
            rc: &mut RenderContext)
            -> Result<(), RenderError> {
    let array_name = h.params().get(0).unwrap();

    let array = c.navigate(rc.get_path(), array_name);
    let default_separator = ", ".to_owned();
    let separator = h.params().get(1).unwrap_or(&default_separator).trim_matches('"');

    let strings = array.as_array()
        .unwrap()
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
