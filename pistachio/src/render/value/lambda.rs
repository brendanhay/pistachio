use std::convert::Infallible;

use crate::{
    render::{
        Context,
        Escape,
        Render,
        Section,
        Writer,
    },
    Template,
};

// struct Lambda {
//     inner: dyn Fn(String) -> String + Send,
// }

/// Equivalent to the following examples:
///
/// * foo()
/// * foo.bar()
/// * foo().bar
/// * foo.bar().baz
///
/// If any value found during the lookup is a callable object, such as a
/// function or lambda, this object will be invoked with zero arguments. The
/// value that is returned is then used instead of the callable object itself.
///

/// It looks like you need special treatment (ie. separate trait impls) based on return type.

/// Has the default section semantics, but needs to error if call like {{foo.bar.baz}}
///
/// An optional part of the specification states that if the final key in the
/// name is a lambda that returns a string, then that string should be rendered
/// as a Mustache template before interpolation. It will be rendered using the
/// default delimiters (see Set Delimiter below) against the current context.

pub struct Source {
    pub source: String,
}

impl Render for Source {
    #[inline]
    fn variable(
        &self,
        _escape: Escape,
        context: Context,
        writer: &mut Writer,
    ) -> Result<(), Infallible> {
        let template = Template::new(&self.source).unwrap();
        let context = context.fork(&template.nodes);

        context.render(writer)
    }
}

impl<T: Render> Render for fn() -> T {
    fn variable(
        &self,
        escape: Escape,
        context: Context,
        writer: &mut Writer,
    ) -> Result<(), Infallible> {
        self().variable(escape, context, writer)
    }

    fn section(
        &self,
        section: Section,
        context: Context,
        writer: &mut Writer,
    ) -> Result<(), Infallible> {
        let frame = self();
        if frame.section_is_truthy(section) {
            context.push(&frame).render(writer)?;
        }

        Ok(())
    }
}

impl<T: Render> Render for fn(String) -> T {
    fn section(
        &self,
        section: Section,
        context: Context,
        writer: &mut Writer,
    ) -> Result<(), Infallible> {
        let source = context.capture()?;
        let frame = self(source);
        if frame.section_is_truthy(section) {
            context.push(&frame).render(writer)?;
        }

        Ok(())
    }
}
