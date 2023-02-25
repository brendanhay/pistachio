use serde_json::{
    Map,
    Number,
    Value,
};

use crate::{
    error::Error,
    render::{
        Context,
        Render,
        Writer,
    },
    Template,
};

impl Render for Value {
    #[inline]
    fn size_hint(&self) -> usize {
        match self {
            Value::Null => ().size_hint(),
            Value::Bool(b) => b.size_hint(),
            Value::Number(n) => n.size_hint(),
            Value::String(s) => s.size_hint(),
            Value::Array(v) => v.size_hint(),
            Value::Object(m) => m.size_hint(),
        }
    }

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
    fn render_escaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        match self {
            Value::Null => ().render_escaped(context, writer),
            Value::Bool(b) => b.render_escaped(context, writer),
            Value::Number(n) => n.render_escaped(context, writer),
            Value::String(s) => s.render_escaped(context, writer),
            Value::Array(v) => v.render_escaped(context, writer),
            Value::Object(m) => m.render_escaped(context, writer),
        }
    }

    #[inline]
    fn render_unescaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        match self {
            Value::Null => ().render_unescaped(context, writer),
            Value::Bool(b) => b.render_unescaped(context, writer),
            Value::Number(n) => n.render_unescaped(context, writer),
            Value::String(s) => s.render_unescaped(context, writer),
            Value::Array(v) => v.render_unescaped(context, writer),
            Value::Object(m) => m.render_unescaped(context, writer),
        }
    }

    #[inline]
    fn render_section(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
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
    fn resolve(&self, key: &str) -> Option<&dyn Render> {
        match self {
            Value::Object(m) => m.resolve(key),
            _ => None,
        }
    }
}

impl Render for Map<String, Value> {
    #[inline]
    fn is_truthy(&self) -> bool {
        !self.is_empty()
    }

    #[inline]
    fn render_section(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        context.push(self).render_to_writer(writer)
    }

    #[inline]
    fn resolve(&self, key: &str) -> Option<&dyn Render> {
        self.get(key).map(|v| v as &dyn Render)
    }
}

impl Render for Number {
    #[inline]
    fn size_hint(&self) -> usize {
        if let Some(n) = self.as_u64() {
            n.size_hint()
        } else if let Some(n) = self.as_i64() {
            n.size_hint()
        } else if let Some(n) = self.as_f64() {
            n.size_hint()
        } else {
            0
        }
    }

    #[inline]
    fn is_truthy(&self) -> bool {
        if let Some(n) = self.as_u64() {
            n.is_truthy()
        } else if let Some(n) = self.as_i64() {
            n.is_truthy()
        } else if let Some(n) = self.as_f64() {
            n.is_truthy()
        } else {
            false
        }
    }

    #[inline]
    fn render_escaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        if let Some(n) = self.as_u64() {
            n.render_escaped(context, writer)
        } else if let Some(n) = self.as_i64() {
            n.render_escaped(context, writer)
        } else if let Some(n) = self.as_f64() {
            n.render_escaped(context, writer)
        } else {
            Ok(())
        }
    }
}
