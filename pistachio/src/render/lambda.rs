use super::{
    Context,
    Render,
    Writer,
};
use crate::{
    Error,
    Template,
};

/// Whether the source string is a mustache template and should be parsed
/// and rendered against the current context.
pub struct Expand(String);

// impl Render for Expand {
//     #[inline]
//     fn render_escaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
//         let template = Template::new(&self.0)?;
//         context.fork(&template).render_to_writer(writer)
//     }
// }

// impl<T: Render> Render for dyn Fn() -> T {
//     #[inline]
//     fn render_escaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
//         self().render_escaped(context, writer)
//     }
// }

// impl<T: Render> Render for dyn Fn(&str) -> T {
//     #[inline]
//     fn render_section(
//         &self,
//         capture: &str,
//         context: Context,
//         writer: &mut Writer,
//     ) -> Result<(), Error> {
//         self(capture).render_escaped(context, writer)
//     }
// }

// impl<T: Render> Render for dyn Fn(Expand) -> T {
//     #[inline]
//     fn render_section(
//         &self,
//         _capture: &str,
//         context: Context,
//         writer: &mut Writer,
//     ) -> Result<(), Error> {
//         let source = context.render(8)?;
//         self(Expand(source)).render_escaped(context, writer)
//     }
// }
