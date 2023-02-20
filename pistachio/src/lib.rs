#![feature(pattern)]
#![warn(clippy::disallowed_types)]

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::{
    borrow::Cow,
    collections::hash_map::Entry,
    ffi::{
        OsStr,
        OsString,
    },
    fs,
    io,
    path::{
        Path,
        PathBuf,
    },
};

#[cfg(feature = "macros")]
pub use pistachio_macros::Render;
#[cfg(feature = "serde_json")]
pub use serde_json::{
    json,
    Value,
};

pub use self::{
    error::Error,
    parser::ParseError,
    render::Render,
    template::Template,
};

mod error;
mod lexer;
mod map;
mod parser;
mod template;

// Exposed for pistachio-macros to use.
#[doc(hidden)]
pub mod render;

/// The caching strategy determining how templates are loaded.
#[derive(Debug, Clone, Copy, Default)]
pub enum Cache {
    /// Cache non-dynamic templates by name, in memory.
    #[default]
    Name,

    // ModifiedTime,
    /// Don't cache templates. Every request for a non-dynamic template
    /// name will cause it to be read from the file system.
    None,
}

// pub enum Flags,

/// XXX: raise on variable miss - raise when partial/parent don't exist, etc.

/// Everybody loves `Pistachio`.
pub struct Pistachio {
    root: PathBuf,
    extension: OsString,
    templates: map::Map<Cow<'static, str>, Template<'static>>,
    cache: Cache,
    // raise: bool,
}

impl Pistachio {
    pub fn new<Dir>(dir: Dir) -> Result<Self, Error>
    where
        Dir: AsRef<Path>,
    {
        Self::configure(Cache::default(), dir, "mustache")
    }

    /// Create a new `Pistachio` using the specified caching strategy, with the specified
    /// root directory and file extension as the search mechanism for loaded templates.
    pub fn configure<Dir, Ext>(cache: Cache, dir: Dir, extension: Ext) -> Result<Self, Error>
    where
        Dir: AsRef<Path>,
        Ext: AsRef<OsStr>,
    {
        Ok(Self {
            root: dir.as_ref().canonicalize()?,
            extension: extension.as_ref().into(),
            templates: map::new(),
            cache,
        })
    }

    /// Get an existing template from this `Pistachio`, reading it from the filesystem
    /// and parsing it, if not already present in memory.
    pub fn get(&mut self, name: &str) -> Result<&Template<'static>, Error> {
        match self.cache {
            Cache::Name if self.templates.contains_key(name) => {},
            Cache::Name => {
                self.read_template(Cow::Owned(name.to_string()))?;
            },
            Cache::None => {
                self.read_template(Cow::Owned(name.to_string()))?;
            },
        }

        Ok(&self.templates[name])
    }

    /// Add a template to this `Pistachio`.
    pub fn add<Name, Source>(
        &mut self,
        name: Name,
        source: Source,
    ) -> Result<&Template<'static>, Error>
    where
        Name: Into<Cow<'static, str>>,
        Source: Into<Cow<'static, str>>,
    {
        let name = name.into();
        let template = Template::with_loader(source.into(), self)?;

        Ok(self.insert_template(name, template))
    }

    #[inline]
    fn read_template(&mut self, name: Cow<'static, str>) -> Result<&Template<'static>, Error> {
        let path = self
            .root
            .join(name.as_ref())
            .with_extension(&self.extension)
            .canonicalize()?;

        if !path.starts_with(&self.root) {
            return Err(Error::InvalidPartial(path.display().to_string().into()));
        }

        let source = match fs::read_to_string(&path) {
            Ok(file) => Ok(file),
            Err(err) => {
                if err.kind() == io::ErrorKind::NotFound {
                    Err(Error::NotFound(name.to_string().into()))
                } else {
                    Err(Error::Io(err))
                }
            },
        }?;

        let template = Template::with_loader(source.into(), self)?;

        Ok(self.insert_template(name, template))
    }

    #[inline]
    fn insert_template(
        &mut self,
        name: Cow<'static, str>,
        template: Template<'static>,
    ) -> &Template<'static> {
        // https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.insert_entry
        match self.templates.entry(name) {
            Entry::Occupied(mut entry) => {
                entry.insert(template);
                &*entry.into_mut()
            },
            Entry::Vacant(entry) => &*entry.insert(template),
        }
    }
}

#[doc(hidden)]
pub trait Loader<'a> {
    /// Invoked as a callback by the LR parser to obtain a child template when
    /// `{{<parent}}` or `{{>partial}}` are encountered.
    fn get_template(&mut self, name: &'a str) -> Result<&Template<'a>, Error>;
}

struct LoadingDisabled;

impl<'a> Loader<'a> for LoadingDisabled {
    fn get_template(&mut self, _name: &'a str) -> Result<&Template<'a>, Error> {
        Err(Error::LoadingDisabled)
    }
}
impl Loader<'static> for Pistachio {
    fn get_template(&mut self, name: &'static str) -> Result<&Template<'static>, Error> {
        if !self.templates.contains_key(name) {
            self.read_template(name.into())?;
        }

        Ok(&self.templates[name])
    }
}

// #[macro_export]
// #[doc(hidden)]
// macro_rules! __context_pair {
//     ($ctx:ident, $key:ident) => {{
//         $crate::__context_pair!($ctx, $key, $key);
//     }};
//     ($ctx:ident, $key:ident, $value:expr) => {
//         $crate::__context::add(
//             &mut $ctx,
//             stringify!($key),
//             $crate::value::Value::from_serializable(&$value),
//         );
//     };
// }

// #[doc(hidden)]
// pub mod __context {
//     use crate::data::Data;

//     #[inline(always)]
//     pub fn new() -> HashMap<String, >{
//         ValueMap::default()
//     }

//     #[inline(always)]
//     pub fn add(ctx: &mut ValueMap, key: &'static str, value: Value) {
//         ctx.insert(Key::Str(key), value);
//     }

//     #[inline(always)]
//     pub fn build(ctx: ValueMap) -> Value {
//         ValueRepr::Map(Arc::new(ctx), MapType::Normal).into()
//     }

//     pub fn thread_local_env() -> Environment<'static> {
//         thread_local! {
//             static ENV: Environment<'static> = Environment::new()
//         }
//         ENV.with(|x| x.clone())
//     }
// }
