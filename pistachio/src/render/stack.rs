use super::{
    Context,
    Escape,
    Render,
    RenderError,
    Writer,
};

pub trait Stack: Sized + Copy {
    type I: Sized + Copy + Render;
    type J: Sized + Copy + Render;
    type K: Sized + Copy + Render;
    type L: Sized + Copy + Render;
    type M: Sized + Copy + Render;

    type Previous: RenderStack;

    fn push<X: ?Sized + Render>(
        self,
        frame: &X,
    ) -> (Self::I, Self::J, Self::K, Self::L, Self::M, &X);

    fn pop(self) -> Self::Previous;
}

pub type PushStack<S, X> = (
    <S as Stack>::I,
    <S as Stack>::J,
    <S as Stack>::K,
    <S as Stack>::L,
    <S as Stack>::M,
    X,
);

impl Stack for () {
    type I = ();
    type J = ();
    type K = ();
    type L = ();
    type M = ();

    type Previous = ();

    #[inline]
    fn push<X: ?Sized>(self, frame: &X) -> ((), (), (), (), (), &X) {
        ((), (), (), (), (), frame)
    }

    #[inline]
    fn pop(self) -> Self::Previous {}
}

impl<A, B, C, D, E, F> Stack for (A, B, C, D, E, F)
where
    A: Copy + Render,
    B: Copy + Render,
    C: Copy + Render,
    D: Copy + Render,
    E: Copy + Render,
    F: Copy + Render,
{
    type I = B;
    type J = C;
    type K = D;
    type L = E;
    type M = F;

    type Previous = ((), A, B, C, D, E);

    #[inline]
    fn push<X: ?Sized + Render>(self, frame: &X) -> (B, C, D, E, F, &X) {
        (self.1, self.2, self.3, self.4, self.5, frame)
    }

    #[inline]
    fn pop(self) -> Self::Previous {
        ((), self.0, self.1, self.2, self.3, self.4)
    }
}

pub trait RenderStack: Sized + Copy + Stack {
    #[inline]
    fn render_stack_escape<W: Writer>(
        &self,
        _key: &str,
        _escape: Escape,
        _writer: &mut W,
    ) -> Result<bool, RenderError<W::Error>> {
        Ok(false)
    }

    #[inline]
    fn render_stack_section<'a, S, W>(
        &self,
        _key: &str,
        _context: Context<'a, S>,
        _writer: &mut W,
    ) -> Result<(), RenderError<W::Error>>
    where
        S: RenderStack,
        W: Writer,
    {
        Ok(())
    }

    #[inline]
    fn render_stack_inverted_section<'a, S, W>(
        &self,
        _key: &str,
        _context: Context<'a, S>,
        _writer: &mut W,
    ) -> Result<(), RenderError<W::Error>>
    where
        S: RenderStack,
        W: Writer,
    {
        Ok(())
    }
}

impl RenderStack for () {}

impl<A, B, C, D, E, F> RenderStack for (A, B, C, D, E, F)
where
    A: Copy + Render,
    B: Copy + Render,
    C: Copy + Render,
    D: Copy + Render,
    E: Copy + Render,
    F: Copy + Render,
{
    #[inline]
    fn render_stack_escape<W: Writer>(
        &self,
        key: &str,
        escape: Escape,
        writer: &mut W,
    ) -> Result<bool, RenderError<W::Error>> {
        if self.5.field_variable(key, escape, writer)?
            || self.4.field_variable(key, escape, writer)?
            || self.3.field_variable(key, escape, writer)?
            || self.2.field_variable(key, escape, writer)?
            || self.1.field_variable(key, escape, writer)?
            || self.0.field_variable(key, escape, writer)?
        {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[inline]
    fn render_stack_section<'a, S, W>(
        &self,
        key: &str,
        context: Context<'a, S>,
        writer: &mut W,
    ) -> Result<(), RenderError<W::Error>>
    where
        S: RenderStack,
        W: Writer,
    {
        if !self.5.field_section(key, context, writer)? {
            let context = context.pop();
            if !self.4.field_section(key, context, writer)? {
                let context = context.pop();
                if !self.3.field_section(key, context, writer)? {
                    let context = context.pop();
                    if !self.2.field_section(key, context, writer)? {
                        let context = context.pop();
                        if !self.1.field_section(key, context, writer)? {
                            let context = context.pop();
                            self.0.field_section(key, context, writer)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    #[inline]
    fn render_stack_inverted_section<'a, S, W>(
        &self,
        key: &str,
        context: Context<'a, S>,
        writer: &mut W,
    ) -> Result<(), RenderError<W::Error>>
    where
        S: RenderStack,
        W: Writer,
    {
        if !self.5.field_inverted_section(key, context, writer)?
            && !self.4.field_inverted_section(key, context, writer)?
            && !self.3.field_inverted_section(key, context, writer)?
            && !self.2.field_inverted_section(key, context, writer)?
            && !self.1.field_inverted_section(key, context, writer)?
            && !self.0.field_inverted_section(key, context, writer)?
        {
            context.render(writer)?;
        }

        Ok(())
    }
}
