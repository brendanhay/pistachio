use super::{
    Context,
    Escape,
    Render,
    Writer,
};

pub trait Stack: Sized + Copy {
    type I: Sized + Copy + Render;
    type J: Sized + Copy + Render;
    type K: Sized + Copy + Render;

    type Previous: RenderStack;

    fn push<X: ?Sized + Render>(self, frame: &X) -> (Self::I, Self::J, Self::K, &X);

    fn pop(self) -> Self::Previous;
}

pub type PushStack<S, X> = (<S as Stack>::I, <S as Stack>::J, <S as Stack>::K, X);

impl Stack for () {
    type I = ();
    type J = ();
    type K = ();

    type Previous = ();

    #[inline]
    fn push<X: ?Sized>(self, frame: &X) -> ((), (), (), &X) {
        ((), (), (), frame)
    }

    #[inline]
    fn pop(self) -> Self::Previous {}
}

impl<A, B, C, D> Stack for (A, B, C, D)
where
    A: Copy + Render,
    B: Copy + Render,
    C: Copy + Render,
    D: Copy + Render,
{
    type I = B;
    type J = C;
    type K = D;

    type Previous = ((), A, B, C);

    #[inline]
    fn push<X: ?Sized + Render>(self, frame: &X) -> (B, C, D, &X) {
        (self.1, self.2, self.3, frame)
    }

    #[inline]
    fn pop(self) -> Self::Previous {
        ((), self.0, self.1, self.2)
    }
}

pub trait RenderStack: Sized + Copy + Stack {
    #[inline]
    fn render_field_escape<W: Writer>(
        &self,
        _key: &str,
        _escape: Escape,
        _writer: &mut W,
    ) -> Result<(), W::Error> {
        Ok(())
    }

    #[inline]
    fn render_field_section<'a, S, W>(
        &self,
        _key: &str,
        _context: Context<'a, S>,
        _writer: &mut W,
    ) -> Result<(), W::Error>
    where
        S: RenderStack,
        W: Writer,
    {
        Ok(())
    }

    #[inline]
    fn render_field_inverted_section<'a, S, W>(
        &self,
        _key: &str,
        _context: Context<'a, S>,
        _writer: &mut W,
    ) -> Result<(), W::Error>
    where
        S: RenderStack,
        W: Writer,
    {
        Ok(())
    }
}

impl RenderStack for () {}

impl<A, B, C, D> RenderStack for (A, B, C, D)
where
    A: Copy + Render,
    B: Copy + Render,
    C: Copy + Render,
    D: Copy + Render,
{
    #[inline]
    fn render_field_escape<W: Writer>(
        &self,
        key: &str,
        escape: Escape,
        writer: &mut W,
    ) -> Result<(), W::Error> {
        if !self.3.render_field_escape(key, escape, writer)?
            && !self.2.render_field_escape(key, escape, writer)?
            && !self.1.render_field_escape(key, escape, writer)?
        {
            self.0.render_field_escape(key, escape, writer)?;
        }

        Ok(())
    }

    #[inline]
    fn render_field_section<'a, S, W>(
        &self,
        key: &str,
        context: Context<'a, S>,
        writer: &mut W,
    ) -> Result<(), W::Error>
    where
        S: RenderStack,
        W: Writer,
    {
        if !self.3.render_field_section(key, context, writer)? {
            let context = context.pop();

            if !self.2.render_field_section(key, context, writer)? {
                let context = context.pop();

                if !self.1.render_field_section(key, context, writer)? {
                    let context = context.pop();

                    self.0.render_field_section(key, context, writer)?;
                }
            }
        }

        Ok(())
    }

    #[inline]
    fn render_field_inverted_section<'a, S, W>(
        &self,
        key: &str,
        context: Context<'a, S>,
        writer: &mut W,
    ) -> Result<(), W::Error>
    where
        S: RenderStack,
        W: Writer,
    {
        if !self.3.render_field_inverted_section(key, context, writer)?
            && !self.2.render_field_inverted_section(key, context, writer)?
            && !self.1.render_field_inverted_section(key, context, writer)?
            && !self.0.render_field_inverted_section(key, context, writer)?
        {
            context.render(writer)?;
        }

        Ok(())
    }
}