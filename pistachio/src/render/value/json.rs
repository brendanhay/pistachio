use serde_json::{
    Map,
    Number,
    Value,
};

use super::map::impl_map;
use crate::{
    render::{
        stack,
        Context,
        Escape,
        Render,
        RenderError,
        Writer,
    },
    Template,
};

impl Render for Value {
    #[inline]
    fn is_truthy(&self) -> bool {
        match self {
            Value::Null => ().is_truthy(),
            Value::Bool(b) => b.is_truthy(),
            Value::Number(n) => n.is_truthy(),
            Value::String(s) => s.is_truthy(),
            Value::Array(v) => v.is_truthy(),
            Value::Object(m) => m.is_truthy(),
        }
    }

    #[inline]
    fn size_hint(&self, template: &Template) -> usize {
        match self {
            Value::Null => ().size_hint(template),
            Value::Bool(b) => b.size_hint(template),
            Value::Number(n) => n.size_hint(template),
            Value::String(s) => s.size_hint(template),
            Value::Array(v) => v.size_hint(template),
            Value::Object(m) => m.size_hint(template),
        }
    }

    #[inline]
    fn render_escape<W: Writer>(
        &self,
        escape: Escape,
        writer: &mut W,
    ) -> Result<(), RenderError<W::Error>> {
        match self {
            Value::Null => ().render_escape(escape, writer),
            Value::Bool(b) => b.render_escape(escape, writer),
            Value::Number(n) => n.render_escape(escape, writer),
            Value::String(s) => s.render_escape(escape, writer),
            Value::Array(v) => v.render_escape(escape, writer),
            Value::Object(m) => m.render_escape(escape, writer),
        }
    }

    #[inline]
    fn render_section<S, W>(
        &self,
        context: Context<S>,
        writer: &mut W,
    ) -> Result<(), RenderError<W::Error>>
    where
        S: stack::RenderStack,
        W: Writer,
    {
        match self {
            Value::Null => ().render_section(context, writer),
            Value::Bool(b) => b.render_section(context, writer),
            Value::Number(n) => n.render_section(context, writer),
            Value::String(s) => s.render_section(context, writer),
            Value::Array(v) => v.render_section(context, writer),
            Value::Object(m) => m.render_section(context, writer),
        }
    }

    #[inline]
    fn render_inverted_section<S, W>(
        &self,
        context: Context<S>,
        writer: &mut W,
    ) -> Result<(), RenderError<W::Error>>
    where
        S: stack::RenderStack,
        W: Writer,
    {
        match self {
            Value::Null => ().render_inverted_section(context, writer),
            Value::Bool(b) => b.render_inverted_section(context, writer),
            Value::Number(n) => n.render_inverted_section(context, writer),
            Value::String(s) => s.render_inverted_section(context, writer),
            Value::Array(v) => v.render_inverted_section(context, writer),
            Value::Object(m) => m.render_inverted_section(context, writer),
        }
    }

    #[inline]
    fn render_field_escape<W: Writer>(
        &self,
        key: &str,
        escape: Escape,
        writer: &mut W,
    ) -> Result<bool, RenderError<W::Error>> {
        match self {
            Value::Null => ().render_field_escape(key, escape, writer),
            Value::Bool(b) => b.render_field_escape(key, escape, writer),
            Value::Number(n) => n.render_field_escape(key, escape, writer),
            Value::String(s) => s.render_field_escape(key, escape, writer),
            Value::Array(v) => v.render_field_escape(key, escape, writer),
            Value::Object(m) => m.render_field_escape(key, escape, writer),
        }
    }

    #[inline]
    fn render_field_section<S, W>(
        &self,
        key: &str,
        context: Context<S>,
        writer: &mut W,
    ) -> Result<bool, RenderError<W::Error>>
    where
        S: stack::RenderStack,
        W: Writer,
    {
        match self {
            Value::Null => ().render_field_section(key, context, writer),
            Value::Bool(b) => b.render_field_section(key, context, writer),
            Value::Number(n) => n.render_field_section(key, context, writer),
            Value::String(s) => s.render_field_section(key, context, writer),
            Value::Array(v) => v.render_field_section(key, context, writer),
            Value::Object(m) => m.render_field_section(key, context, writer),
        }
    }

    #[inline]
    fn render_field_inverted_section<S, W>(
        &self,
        key: &str,
        context: Context<S>,
        writer: &mut W,
    ) -> Result<bool, RenderError<W::Error>>
    where
        S: stack::RenderStack,
        W: Writer,
    {
        match self {
            Value::Null => ().render_field_inverted_section(key, context, writer),
            Value::Bool(b) => b.render_field_inverted_section(key, context, writer),
            Value::Number(n) => n.render_field_inverted_section(key, context, writer),
            Value::String(s) => s.render_field_inverted_section(key, context, writer),
            Value::Array(v) => v.render_field_inverted_section(key, context, writer),
            Value::Object(m) => m.render_field_inverted_section(key, context, writer),
        }
    }
}

impl Render for Map<String, Value> {
    impl_map! {}
}

impl Render for Number {
    #[inline]
    fn is_truthy(&self) -> bool {
        if let Some(n) = self.as_f64() {
            n.is_truthy()
        } else if let Some(n) = self.as_u64() {
            n.is_truthy()
        } else if let Some(n) = self.as_i64() {
            n.is_truthy()
        } else {
            false
        }
    }

    #[inline]
    fn size_hint(&self, template: &Template) -> usize {
        if let Some(n) = self.as_f64() {
            n.size_hint(template)
        } else if let Some(n) = self.as_u64() {
            n.size_hint(template)
        } else if let Some(n) = self.as_i64() {
            n.size_hint(template)
        } else {
            0
        }
    }

    #[inline]
    fn render_escape<W: Writer>(
        &self,
        escape: Escape,
        writer: &mut W,
    ) -> Result<(), RenderError<W::Error>> {
        if let Some(n) = self.as_f64() {
            n.render_escape(escape, writer)
        } else if let Some(n) = self.as_u64() {
            n.render_escape(escape, writer)
        } else if let Some(n) = self.as_i64() {
            n.render_escape(escape, writer)
        } else {
            Ok(())
        }
    }
}
