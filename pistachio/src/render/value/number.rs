use crate::{
    error::Error,
    render::{
        Context,
        Render,
        Writer,
    },
    Template,
};

macro_rules! impl_numbers {
    ( $( $ty:ty ),* ) => {
        $(
            impl Render for $ty {
                #[inline]
                fn size_hint(&self, _template: &Template) -> usize {
                    5
                }

                #[inline]
                fn is_truthy(&self) -> bool {
                    *self != 0 as $ty
                }

                #[inline]
                fn render(
                    &self,
                    context: Context,
                    writer: &mut Writer
                ) -> Result<(), Error> {
                    writer.format(context.escape, self)
                }
            }
        )*
    };
}

impl_numbers!(u8, u16, u32, u64, u128, usize);
impl_numbers!(i8, i16, i32, i64, i128, isize);

macro_rules! impl_float {
    ( $($ty:ty : $epsilon:path),* ) => {
        $(
            impl Render for $ty  {
                #[inline]
                fn size_hint(&self, _template: &Template) -> usize {
                    5
                }

                #[inline]
                fn is_truthy(&self) -> bool {
                    self.abs() > $epsilon
                }

                #[inline]
                fn render(
                    &self,
                    context: Context,
                    writer: &mut Writer
                ) -> Result<(), Error> {
                    writer.format(context.escape, self)
                }
            }
        )*
    };
}

impl_float!(f32: f32::EPSILON, f64: f64::EPSILON);
