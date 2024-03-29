use crate::{
    error::Error,
    render::{
        Context,
        Render,
        Writer,
    },
};

macro_rules! impl_sequence {
    () => {
        #[inline]
        fn is_truthy(&self) -> bool {
            !self.is_empty()
        }

        #[inline]
        fn render_section(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
            println!("sequence.render_section: {:?}", self);

            for item in self.iter() {
                item.render_section(context.push(item), writer)?;
            }

            Ok(())
        }
    };
}

impl<T: Render> Render for Vec<T> {
    impl_sequence! {}
}

impl<T: Render> Render for [T] {
    impl_sequence! {}
}

impl<T: Render, const N: usize> Render for [T; N] {
    impl_sequence! {}
}
