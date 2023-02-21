use std::convert::Infallible;

use crate::{
    render::{
        Context,
        Render,
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

impl<T: Render> Render for fn() -> T {
    fn render(&self, context: Context, writer: &mut Writer) -> Result<(), Infallible> {
        self().render(context, writer)
    }

    fn render_section(&self, context: Context, writer: &mut Writer) -> Result<(), Infallible> {
        let frame = self();
        if frame.section_is_truthy(context.section) {
            context.push(&frame).render(writer)?;
        }

        Ok(())
    }
}

// Box<dyn Fn() -> T>
impl<T: Render> Render for dyn Fn() -> T {
    fn render(&self, context: Context, writer: &mut Writer) -> Result<(), Infallible> {
        self().render(context, writer)
    }

    fn render_section(&self, context: Context, writer: &mut Writer) -> Result<(), Infallible> {
        let frame = self();
        if frame.section_is_truthy(context.section) {
            context.push(&frame).render(writer)?;
        }

        Ok(())
    }
}

impl<T: Render> Render for fn(String) -> T {
    fn render_section(&self, context: Context, writer: &mut Writer) -> Result<(), Infallible> {
        let source = context.capture()?;
        let frame = self(source);
        if frame.section_is_truthy(context.section) {
            context.push(&frame).render(writer)?;
        }

        Ok(())
    }
}

// Box<dyn Fn(String) -> T>
impl<T: Render> Render for dyn Fn(String) -> T {
    fn render_section(&self, context: Context, writer: &mut Writer) -> Result<(), Infallible> {
        let source = context.capture()?;
        let frame = self(source);
        if frame.section_is_truthy(context.section) {
            context.push(&frame).render(writer)?;
        }

        Ok(())
    }
}
