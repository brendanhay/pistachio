use std::{
    convert,
    iter,
};

use super::{
    Context,
    Escape,
    Render,
    RenderError,
    Writer,
};
use crate::Template;

pub struct Frame<'a, X> {
    pub name: &'a str,
    pub data: &'a X,
}

impl<X> Copy for Frame<'_, X> {}

impl<X> Clone for Frame<'_, X> {
    fn clone(&self) -> Self {
        *self
    }
}

pub trait Trace {
    fn trace(&self) -> Option<&str>;
}

impl Trace for () {
    fn trace(&self) -> Option<&str> {
        None
    }
}

impl<T> Trace for Frame<'_, T> {
    fn trace(&self) -> Option<&str> {
        Some(self.name)
    }
}

pub trait Stack: Sized + Copy {
    type I: Sized + Copy + Render + Trace;
    type J: Sized + Copy + Render + Trace;
    type K: Sized + Copy + Render + Trace;
    type L: Sized + Copy + Render + Trace;
    type M: Sized + Copy + Render + Trace;

    type Previous: RenderStack;

    fn push<X: Render>(self, frame: X) -> (Self::I, Self::J, Self::K, Self::L, Self::M, X);

    fn pop(self) -> Self::Previous;

    fn trace(&self) -> Vec<&str>;
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
    fn push<X>(self, frame: X) -> ((), (), (), (), (), X) {
        ((), (), (), (), (), frame)
    }

    #[inline]
    fn pop(self) -> Self::Previous {}

    #[inline]
    fn trace(&self) -> Vec<&str> {
        Vec::new()
    }
}

impl<A, B, C, D, E, F> Stack for (A, B, C, D, E, F)
where
    A: Copy + Render + Trace,
    B: Copy + Render + Trace,
    C: Copy + Render + Trace,
    D: Copy + Render + Trace,
    E: Copy + Render + Trace,
    F: Copy + Render + Trace,
{
    type I = B;
    type J = C;
    type K = D;
    type L = E;
    type M = F;

    type Previous = ((), A, B, C, D, E);

    #[inline]
    fn push<X: Render>(self, frame: X) -> (B, C, D, E, F, X) {
        (self.1, self.2, self.3, self.4, self.5, frame)
    }

    #[inline]
    fn pop(self) -> Self::Previous {
        ((), self.0, self.1, self.2, self.3, self.4)
    }

    #[inline]
    fn trace(&self) -> Vec<&str> {
        self.0
            .trace()
            .into_iter()
            .chain(self.1.trace().into_iter())
            .chain(self.2.trace().into_iter())
            .chain(self.3.trace().into_iter())
            .chain(self.4.trace().into_iter())
            .chain(self.5.trace().into_iter())
            .collect()
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
    A: Copy + Render + Trace,
    B: Copy + Render + Trace,
    C: Copy + Render + Trace,
    D: Copy + Render + Trace,
    E: Copy + Render + Trace,
    F: Copy + Render + Trace,
{
    #[inline]
    fn render_stack_escape<W: Writer>(
        &self,
        key: &str,
        escape: Escape,
        writer: &mut W,
    ) -> Result<bool, RenderError<W::Error>> {
        if self.5.render_field_escape(key, escape, writer)?
            || self.4.render_field_escape(key, escape, writer)?
            || self.3.render_field_escape(key, escape, writer)?
            || self.2.render_field_escape(key, escape, writer)?
            || self.1.render_field_escape(key, escape, writer)?
            || self.0.render_field_escape(key, escape, writer)?
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
        if !self.5.render_field_section(key, context, writer)? {
            let context = context.pop();
            if !self.4.render_field_section(key, context, writer)? {
                let context = context.pop();
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
        if !self.5.render_field_inverted_section(key, context, writer)?
            && !self.4.render_field_inverted_section(key, context, writer)?
            && !self.3.render_field_inverted_section(key, context, writer)?
            && !self.2.render_field_inverted_section(key, context, writer)?
            && !self.1.render_field_inverted_section(key, context, writer)?
            && !self.0.render_field_inverted_section(key, context, writer)?
        {
            context.render(writer)?;
        }

        Ok(())
    }
}

impl<'a, T: Render> Render for Frame<'a, T> {
    #[inline]
    fn is_truthy(&self) -> bool {
        self.data.is_truthy()
    }

    #[inline]
    fn size_hint(&self, template: &Template) -> usize {
        self.data.size_hint(template)
    }

    #[inline]
    fn render_escape<W: Writer>(
        &self,
        escape: Escape,
        writer: &mut W,
    ) -> Result<(), RenderError<W::Error>> {
        self.data.render_escape(escape, writer)
    }

    #[inline]
    fn render_section<S, W>(
        &self,
        key: &str,
        context: Context<S>,
        writer: &mut W,
    ) -> Result<(), RenderError<W::Error>>
    where
        S: RenderStack,
        W: Writer,
    {
        debug_assert!(key == self.name);

        self.data.render_section(key, context, writer)
    }

    #[inline]
    fn render_inverted_section<S, W>(
        &self,
        key: &str,
        context: Context<S>,
        writer: &mut W,
    ) -> Result<(), RenderError<W::Error>>
    where
        S: RenderStack,
        W: Writer,
    {
        debug_assert!(key == self.name);

        self.data.render_inverted_section(key, context, writer)
    }

    #[inline]
    fn render_field_escape<W: Writer>(
        &self,
        key: &str,
        escape: Escape,
        writer: &mut W,
    ) -> Result<bool, RenderError<W::Error>> {
        self.data.render_field_escape(key, escape, writer)
    }

    #[inline]
    fn render_field_section<S, W>(
        &self,
        key: &str,
        context: Context<S>,
        writer: &mut W,
    ) -> Result<bool, RenderError<W::Error>>
    where
        S: RenderStack,
        W: Writer,
    {
        self.data.render_field_section(key, context, writer)
    }

    #[inline]
    fn render_field_inverted_section<S, W>(
        &self,
        key: &str,
        context: Context<S>,
        writer: &mut W,
    ) -> Result<bool, RenderError<W::Error>>
    where
        S: RenderStack,
        W: Writer,
    {
        self.data
            .render_field_inverted_section(key, context, writer)
    }
}
