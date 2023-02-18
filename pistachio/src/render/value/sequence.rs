use crate::render::{
    Context,
    Render,
    RenderStack,
    Writer,
};

macro_rules! impl_sequence {
    () => {
        #[inline]
        fn is_truthy(&self) -> bool {
            !self.is_empty()
        }

        #[inline]
        fn render_section<S, W>(&self, context: Context<S>, writer: &mut W) -> Result<(), W::Error>
        where
            S: RenderStack,
            W: Writer,
        {
            for item in self.iter() {
                item.render_section(context, writer)?;
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
