use crate::{
    render::{
        Escape,
        Render,
        RenderError,
        Writer,
    },
    Template,
};

macro_rules! impl_numbers {
    ( $( $ty:ty ),* ) => {
        $(
            impl Render for $ty {
                #[inline]
                fn is_truthy(&self) -> bool {
                    *self != 0 as $ty
                }

                #[inline]
                fn size_hint(&self, _template: &Template) -> usize {
                    5
                }

                #[inline]
                fn render_escape<W: Writer>(
                    &self,
                    _escape: Escape,
                    writer: &mut W
                ) -> Result<(), RenderError<W::Error>> {
                    writer.format_escape(Escape::None, self).map_err(Into::into)
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
                fn is_truthy(&self) -> bool {
                    self.abs() > $epsilon
                }

                #[inline]
                fn size_hint(&self, _template: &Template) -> usize {
                    5
                }

                #[inline]
                fn render_escape<W: Writer>(
                    &self,
                    _escape: Escape,
                    writer: &mut W,
                ) -> Result<(), RenderError<W::Error>> {
                    writer.format_escape(Escape::None, self).map_err(Into::into)
                }
            }
        )*
    };
}

impl_float!(f32: f32::EPSILON, f64: f64::EPSILON);
