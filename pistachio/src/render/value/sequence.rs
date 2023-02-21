use std::convert::Infallible;

use crate::render::{
    Context,
    Render,
    Section,
    Writer,
};

macro_rules! impl_sequence {
    () => {
        #[inline]
        fn is_truthy(&self) -> bool {
            !self.is_empty()
        }

        #[inline]
        fn section(
            &self,
            section: Section,
            context: Context,
            writer: &mut Writer,
        ) -> Result<(), Infallible> {
            if self.section_is_truthy(section) {
                for item in self.iter() {
                    if item.is_truthy() {
                        context.render(writer)?;
                    }
                }
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
