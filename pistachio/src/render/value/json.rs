use serde_json::{
    Map,
    Number,
    Value,
};

use super::map::impl_map;
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
        // println!("json:render_escaped {:?}", self);

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
        // println!("json:render_unescaped {:?}", self);

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
        // println!("json:render_section {:?}", self);

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
    fn render_inverted(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        // println!("json:render_inverted {:?}", self);

        match self {
            Value::Null => ().render_inverted(context, writer),
            Value::Bool(b) => b.render_inverted(context, writer),
            Value::Number(n) => n.render_inverted(context, writer),
            Value::String(s) => s.render_inverted(context, writer),
            Value::Array(v) => v.render_inverted(context, writer),
            Value::Object(m) => m.render_inverted(context, writer),
        }
    }

    #[inline]
    fn render_named_escaped(
        &self,
        name: &[&str],
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        // println!("json:render_named_escaped {:?}", self);

        match self {
            Value::Null => ().render_named_escaped(name, context, writer),
            Value::Bool(b) => b.render_named_escaped(name, context, writer),
            Value::Number(n) => n.render_named_escaped(name, context, writer),
            Value::String(s) => s.render_named_escaped(name, context, writer),
            Value::Array(v) => v.render_named_escaped(name, context, writer),
            Value::Object(m) => m.render_named_escaped(name, context, writer),
        }
    }

    #[inline]
    fn render_named_unescaped(
        &self,
        name: &[&str],
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        // println!("json:render_named_unescaped {:?}", self);

        match self {
            Value::Null => ().render_named_unescaped(name, context, writer),
            Value::Bool(b) => b.render_named_unescaped(name, context, writer),
            Value::Number(n) => n.render_named_unescaped(name, context, writer),
            Value::String(s) => s.render_named_unescaped(name, context, writer),
            Value::Array(v) => v.render_named_unescaped(name, context, writer),
            Value::Object(m) => m.render_named_unescaped(name, context, writer),
        }
    }

    #[inline]
    fn render_named_section(
        &self,
        name: &[&str],
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        // println!("json:render_named_section {:?}", self);

        match self {
            Value::Null => ().render_named_section(name, context, writer),
            Value::Bool(b) => b.render_named_section(name, context, writer),
            Value::Number(n) => n.render_named_section(name, context, writer),
            Value::String(s) => s.render_named_section(name, context, writer),
            Value::Array(v) => v.render_named_section(name, context, writer),
            Value::Object(m) => m.render_named_section(name, context, writer),
        }
    }

    #[inline]
    fn render_named_inverted(
        &self,
        name: &[&str],
        context: Context,
        writer: &mut Writer,
    ) -> Result<bool, Error> {
        // println!("json:render_named_inverted {} {:?}", name, self);

        match self {
            Value::Null => ().render_named_inverted(name, context, writer),
            Value::Bool(b) => b.render_named_inverted(name, context, writer),
            Value::Number(n) => n.render_named_inverted(name, context, writer),
            Value::String(s) => s.render_named_inverted(name, context, writer),
            Value::Array(v) => v.render_named_inverted(name, context, writer),
            Value::Object(m) => m.render_named_inverted(name, context, writer),
        }
    }
}

impl Render for Map<String, Value> {
    impl_map! {}
}

impl Render for Number {
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
    fn render_escaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
        println!("number:render_escaped {:?}", self);
        if let Some(n) = self.as_f64() {
            n.render_escaped(context, writer)
        } else if let Some(n) = self.as_u64() {
            n.render_escaped(context, writer)
        } else if let Some(n) = self.as_i64() {
            n.render_escaped(context, writer)
        } else {
            Ok(())
        }
    }
}
