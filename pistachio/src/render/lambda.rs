use super::{
    Context,
    Render,
    Writer,
};
use crate::{
    render::WriteEscaped,
    Error,
    Template,
};

/// Whether the source string is a mustache template and should be parsed
/// and rendered against the current context.
pub struct Expand(pub String);

impl From<&str> for Expand {
    fn from(value: &str) -> Self {
        Expand(value.to_owned())
    }
}

impl From<String> for Expand {
    fn from(value: String) -> Self {
        Expand(value)
    }
}

impl Render for Expand {
    #[inline]
    fn render_escaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        let template = Template::new(&self.0)?;
        let buffer = context.fork(&template).render(8)?;

        writer.write_escaped(&buffer)
    }

    #[inline]
    fn render_unescaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        let template = Template::new(&self.0)?;
        let buffer = context.fork(&template).render(8)?;

        writer.write_unescaped(&buffer)
    }
}

impl<T: Render> Render for dyn Fn() -> T {
    #[inline]
    fn render_escaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        self().render_escaped(context, writer)
    }

    #[inline]
    fn render_unescaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        self().render_unescaped(context, writer)
    }
}

impl<T: Render> Render for dyn Fn(&str) -> T {
    #[inline]
    fn render_section(
        &self,
        capture: &str,
        context: Context,
        writer: &mut Writer,
    ) -> Result<(), Error> {
        self(capture).render_escaped(context, writer)
    }
}

impl<T: Render> Render for dyn Fn(Expand) -> T {
    #[inline]
    fn render_section(
        &self,
        _capture: &str,
        context: Context,
        writer: &mut Writer,
    ) -> Result<(), Error> {
        let source = context.render(8)?;
        self(Expand(source)).render_escaped(context, writer)
    }
}
