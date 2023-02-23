#![feature(pattern)]
#![warn(clippy::disallowed_types)]
use std::{
    borrow::Cow,
    collections::hash_map::Entry,
    ffi::{
        OsStr,
        OsString,
    },
    fs,
    path::{
        Path,
        PathBuf,
    },
};

#[cfg(feature = "derive")]
pub use pistachio_derive::Render;

// #[cfg(feature = "serde_json")]
// pub use serde_json::{
//     json,
//     Value,
// };
//
pub use self::{
    error::Error,
    map::Map,
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
#[derive(Debug, Clone, Copy)]
pub enum Cache {
    /// Cache non-dynamic templates by name, in memory.
    Name,

    // ModifiedTime,
    /// Don't cache templates. Every request for a non-dynamic template
    /// name will cause it to be read from the file system.
    None,
}

#[derive(Debug)]
pub struct Builder {
    directory: PathBuf,
    extension: OsString,
    cache: Cache,
    raise: bool,
}

impl Builder {
    pub fn build(self) -> Result<Pistachio, Error> {
        Ok(Pistachio {
            directory: self.directory.canonicalize().map_err(Error::Io)?,
            extension: self.extension,
            templates: map::with_capacity(4),
            cache: self.cache,
            raise: self.raise,
        })
    }

    pub fn directory<P: AsRef<Path>>(mut self, dir: P) -> Self {
        self.directory = dir.as_ref().into();
        self
    }

    pub fn extension<E: AsRef<OsStr>>(mut self, ext: E) -> Self {
        self.extension = ext.as_ref().into();
        self
    }

    pub fn reloading(mut self) -> Self {
        self.cache = Cache::None;
        self
    }

    pub fn missing_is_false(mut self) -> Self {
        self.raise = false;
        self
    }
}

/// Everybody loves `Pistachio`.
#[derive(Debug)]
pub struct Pistachio {
    directory: PathBuf,
    extension: OsString,
    templates: map::Map<Cow<'static, str>, Template<'static>>,
    cache: Cache,
    raise: bool,
}

impl Pistachio {
    /// Create a new `Pistachio` with a `.mustache` file extension and the specified
    /// root directory as the search mechanism for loading templates. Templates will
    /// be parsed once and then cached in memory. If you want to reload templates
    /// configure the caching strategy via [`Builder::reloading`].
    ///
    /// By default missing `{{key}}` variables will raise an error. To change this
    /// behaviour, see [`Builder::missing_is_false`].
    pub fn builder() -> Builder {
        Builder {
            directory: "examples".into(),
            extension: "mustache".into(),
            cache: Cache::Name,
            raise: true,
        }
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
            .directory
            .join(name.as_ref())
            .with_extension(&self.extension)
            .canonicalize()
            .map_err(Error::Io)?;

        if !path.starts_with(&self.directory) {
            return Err(Error::InvalidPartial(path.display().to_string()));
        }

        let source = fs::read_to_string(&path).map_err(Error::Io)?;
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

pub trait Loader<'a> {
    /// Invoked as a callback by the LR parser to obtain a child template when
    /// `{{<parent}}` or `{{>partial}}` are encountered.
    fn get_template(&mut self, name: &'a str) -> Result<&Template<'a>, Error>;

    /// If missing `{{foo}}` variables should raise an error.
    fn raise_if_missing(&self) -> bool {
        false
    }
}

pub struct LoadingDisabled;

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

    fn raise_if_missing(&self) -> bool {
        self.raise
    }
}
