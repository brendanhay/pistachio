#![feature(pattern)]
#![warn(clippy::disallowed_types)]

use std::{
    borrow::{
        Borrow,
        Cow,
    },
    ffi::{
        OsStr,
        OsString,
    },
    fmt,
    fs,
    hash::Hash,
    io,
    path::{
        Path,
        PathBuf,
    },
};

pub use self::{
    error::Error,
    template::Template,
};
use self::{
    map::Map,
    render::{
        Context,
        Render,
        Writer,
    },
};

mod error;
mod lexer;
mod map;
mod parser;
mod template;

pub mod render;

pub(crate) type Templates = Map<Cow<'static, str>, Template<'static>>;

/// A mustache template obtained from a `Pistachio` that potentially references other templates.
pub struct TemplateGuard<'a> {
    pistachio: &'a Pistachio,
    template: &'a Template<'static>,
}

impl fmt::Debug for TemplateGuard<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.template.fmt(f)
    }
}

impl<'a> TemplateGuard<'a> {
    pub fn size_hint(&self) -> usize {
        self.template.size_hint()
    }

    pub fn source(&self) -> &str {
        &self.template.source()
    }

    pub fn render<T>(&self, value: T) -> Result<String, Error>
    where
        T: Render,
    {
        let mut capacity = self.template.size_hint() + value.size_hint();

        // Add 25% for escaping and various expansions.
        capacity += capacity / 4;

        Context::new(
            self.pistachio.raise,
            &self.pistachio.templates,
            &self.template.tags(),
        )
        .push(&value)
        .render(capacity)
    }

    pub fn render_to_writer<T, W>(&self, value: T, writer: &mut W) -> Result<(), Error>
    where
        T: Render,
        W: io::Write,
    {
        let mut writer = Writer::new(writer);

        Context::new(
            self.pistachio.raise,
            &self.pistachio.templates,
            &self.template.tags(),
        )
        .push(&value)
        .render_to_writer(&mut writer)
    }
}

#[derive(Debug)]
pub struct Builder {
    directory: PathBuf,
    extension: OsString,
    cache: bool,
    raise: bool,
}

impl Builder {
    pub fn build(self) -> Result<Pistachio, Error> {
        Ok(Pistachio {
            directory: self.directory.canonicalize().map_err(Error::io)?,
            extension: self.extension,
            templates: map::with_capacity(2),
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

    pub fn disable_caching(mut self) -> Self {
        self.cache = false;
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
    templates: Templates,
    cache: bool,
    raise: bool,
}

impl Pistachio {
    /// New context.
    pub fn new<P: AsRef<Path>>(directory: P) -> Result<Self, Error> {
        Self::builder().directory(directory).build()
    }

    /// Create a new `Pistachio` with a `.mustache` file extension and the specified
    /// root directory as the search mechanism for loading templates. Templates will
    /// be parsed once and then cached in memory. If you want to reload templates
    /// configure the caching strategy via [`Builder::reloading`].
    ///
    /// By default missing `{{key}}` variables will raise an error. To change this
    /// behaviour, see [`Builder::missing_is_false`].
    pub fn builder() -> Builder {
        Builder {
            directory: ".".into(),
            extension: "mustache".into(),
            cache: true,
            raise: true,
        }
    }

    /// Get a template either from memory or by reading from the filesystem (if enabled).
    pub fn get<N>(&mut self, name: N) -> Result<TemplateGuard, Error>
    where
        for<'a> Cow<'a, str>: Borrow<N>,
        N: Eq + Hash + Clone + Into<Cow<'static, str>>,
    {
        if !self.cache || !self.templates.contains_key(&name) {
            // Don't honor self.raise when trying to load a specifically requested template.
            self.load_template(name.to_owned().into(), true)?;
        }

        Ok(TemplateGuard {
            pistachio: &*self,
            template: &self.templates[&name],
        })
    }

    /// Add a template, potentially replacing an existing template with the same name.
    pub fn insert<S>(&mut self, name: &str, source: S) -> Result<TemplateGuard, Error>
    where
        S: Into<Cow<'static, str>>,
    {
        self.insert_template(name.to_owned().into(), source.into())?;

        Ok(TemplateGuard {
            pistachio: &*self,
            template: &self.templates[name],
        })
    }

    #[inline]
    fn insert_template(
        &mut self,
        name: Cow<'static, str>,
        source: Cow<'static, str>,
    ) -> Result<(), Error> {
        let (template, partials) = Template::load(source.into())?;
        self.templates.insert(name, template);
        for partial in partials {
            if !self.templates.contains_key(partial) {
                self.load_template(partial.into(), self.raise)?;
            }
        }

        Ok(())
    }

    #[inline]
    fn load_template(&mut self, name: Cow<'static, str>, raise: bool) -> Result<(), Error> {
        match self.read_file(name.as_ref()) {
            Ok(source) => self.insert_template(name, source.into()),
            Err(Error::NotFound) if !raise => {
                self.templates.insert(name, Template::empty());
                Ok(())
            },
            Err(error) => Err(error),
        }
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
}
