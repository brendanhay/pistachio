#![feature(pattern)]
#![warn(clippy::disallowed_types)]

use std::{
    borrow::Cow,
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
use template::Loader;

use self::map::{
    Map,
    Set,
};
pub use self::{
    error::Error,
    render::{
        Expand,
        Render,
    },
    template::Template,
};

mod error;
mod lexer;
mod map;
mod parser;
mod template;

pub mod render;

#[derive(Debug)]
pub struct Builder {
    directory: PathBuf,
    extension: OsString,
    raise: bool,
}

impl Builder {
    pub fn build(self) -> Result<Pistachio, Error> {
        Ok(Pistachio {
            directory: self.directory.canonicalize().map_err(Error::io)?,
            extension: self.extension,
            templates: map::with_capacity(2),
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

    pub fn raise(mut self) -> Self {
        self.raise = true;
        self
    }
}

/// Everybody loves `Pistachio`.
#[derive(Debug)]
pub struct Pistachio {
    directory: PathBuf,
    extension: OsString,
    templates: Map<Cow<'static, str>, Template<'static>>,
    raise: bool,
}

impl Pistachio {
    /// New context.
    pub fn new<P: AsRef<Path>>(directory: P) -> Result<Self, Error> {
        Self::builder().directory(directory).build()
    }

    /// Create a new `Pistachio` with a `.mustache` file extension and the specified
    /// root directory as the search mechanism for loading templates. Templates will
    /// be parsed once and then cached in memory.
    ///
    /// By default missing `{{key}}` variables are treated as false. To change this
    /// behaviour and raise an error, see [`Builder::raise`].
    pub fn builder() -> Builder {
        Builder {
            directory: ".".into(),
            extension: "mustache".into(),
            raise: false,
        }
    }

    /// Get a template either from memory or by reading from the filesystem (if enabled).
    pub fn get<N: Into<Cow<'static, str>>>(&mut self, name: N) -> Result<&Template, Error> {
        let name = name.into();

        // XXX: Don't honor self.raise when trying to load a specifically requested template.
        if !self.templates.contains_key(&name) {
            self.loader().get_template(name.to_owned())?;
        }

        Ok(&self.templates[&name])
    }

    /// Add a template, potentially replacing an existing template with the same name.
    pub fn insert<S>(&mut self, name: &str, source: S) -> Result<&Template, Error>
    where
        S: Into<Cow<'static, str>>,
    {
        let template = Template::with_loader(source.into(), &mut self.loader())?;
        self.templates.insert(name.to_owned().into(), template);

        Ok(&self.templates[name])
    }

    #[inline]
    fn read_file(&mut self, name: &str) -> Result<String, Error> {
        let path = self
            .directory
            .join(name)
            .with_extension(&self.extension)
            .canonicalize()
            .map_err(Error::io)?;

        if !path.starts_with(&self.directory) {
            return Err(Error::InvalidPartial(path.display().to_string()));
        }

        fs::read_to_string(&path).map_err(Error::io)
    }

    fn loader(&mut self) -> NonRecursive {
        NonRecursive {
            inner: self,
            chain: Set::default(),
        }
    }
}

struct NonRecursive<'a> {
    inner: &'a mut Pistachio,
    chain: Set<String>,
}

impl<'a> Loader<'static> for NonRecursive<'a> {
    fn get_template(&mut self, name: Cow<'static, str>) -> Result<&Template<'static>, Error> {
        if !self.inner.templates.contains_key(&name) {
            if !self.chain.insert(name.to_string()) {
                return Err(Error::RecursivePartial(name.to_string()));
            }

            match self.inner.read_file(name.as_ref()) {
                Ok(source) => {
                    let template = Template::with_loader(source.into(), self)?;
                    self.inner.templates.insert(name.to_owned(), template);
                },
                Err(Error::NotFound) if !self.raise() => {
                    self.inner
                        .templates
                        .insert(name.to_owned(), Template::empty());
                },
                Err(error) => return Err(error),
            }
        }

        Ok(&self.inner.templates[&name])
    }

    fn raise(&self) -> bool {
        self.inner.raise
    }
}
