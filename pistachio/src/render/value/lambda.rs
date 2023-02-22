use crate::{
    error::Error,
    render::{
        Context,
        Render,
        Writer,
    },
};

macro_rules! impl_arity_zero {
    ( $($ty:ty),* ) => {
        $(
            impl<T: Render> Render for $ty {
                #[inline]
                fn render_escaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
                    self().render_escaped(context, writer)
                }

                #[inline]
                fn render_unescaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
                    self().render_unescaped(context, writer)
                }

                #[inline]
                fn render_section(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
                    let result = self();
                    if result.is_truthy() {
                        context.push(&result).render_to_writer(writer)?;
                    }

                    Ok(())
                }

                #[inline]
                fn render_inverted(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
                    let result = self();
                    if !result.is_truthy() {
                        context.push(&result).render_to_writer(writer)?;
                    }

                    Ok(())
                }
            }
        )*
    };
}

impl_arity_zero!(fn() -> T, dyn Fn() -> T);

macro_rules! impl_arity_string {
    ( $($ty:ty),* ) => {
        $(
            impl<T: Render> Render for $ty {
                #[inline]
                fn render_section(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
                    let source = context.render(8)?;
                    let result = self(source);
                    if result.is_truthy() {
                        context.push(&result).render_to_writer(writer)?;
                    }

                    Ok(())
                }
            }
        )*
    };
}

impl_arity_string!(fn(String) -> T, dyn Fn(String) -> T);
