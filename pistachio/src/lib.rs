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

#[cfg(feature = "derive")]
pub use pistachio_derive::Render;

use self::map::{
    Map,
    Set,
};
pub use self::{
    error::Error,
    render::Render,
    template::Template,
};

mod error;
mod lexer;
mod map;
mod parser;
mod template;

pub mod render;

pub(crate) type Templates = Map<Cow<'static, str>, Template<'static>>;

// /// A mustache template obtained from a `Pistachio` that potentially references other templates.
// pub struct TemplateGuard<'a> {
//     pistachio: &'a Pistachio,
//     template: &'a Template<'static>,
// }

// impl fmt::Debug for TemplateGuard<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         self.template.fmt(f)
//     }
// }

// impl<'a> TemplateGuard<'a> {
//     pub fn size_hint(&self) -> usize {
//         self.template.size_hint()
//     }

//     pub fn source(&self) -> &str {
//         &self.template.source()
//     }

//     pub fn render<T>(&self, value: T) -> Result<String, Error>
//     where
//         T: Render,
//     {
//         let mut capacity = self.template.size_hint() + value.size_hint();

//         // Add 25% for escaping and various expansions.
//         capacity += capacity / 4;

//         Context::new(
//             self.pistachio.raise,
//             &self.pistachio.templates,
//             &self.template.tags(),
//         )
//         .push(&value)
//         .render(capacity)
//     }

//     pub fn render_to_writer<T, W>(&self, value: T, writer: &mut W) -> Result<(), Error>
//     where
//         T: Render,
//         W: io::Write,
//     {
//         let mut writer = Writer::new(writer);

//         Context::new(
//             self.pistachio.raise,
//             &self.pistachio.templates,
//             &self.template.tags(),
//         )
//         .push(&value)
//         .render_to_writer(&mut writer)
//     }
// }

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
    pub fn get<N: Into<Cow<'static, str>>>(&mut self, name: N) -> Result<&Template, Error> {
        let name = name.into();

        // XXX: Don't honor self.raise when trying to load a specifically requested template.
        if !self.cache || !self.templates.contains_key(&name) {
            self.loader().get_partial(name.to_owned())?;
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
        println!("read_file: {}", name);

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

#[doc(hidden)]
pub trait Loader<'a> {
    fn get_partial(&mut self, name: Cow<'a, str>) -> Result<&Template<'a>, Error>;

    fn raise(&self) -> bool {
        false
    }
}

pub(crate) struct NoLoading;

impl<'a> Loader<'a> for NoLoading {
    fn get_partial(&mut self, _name: Cow<'a, str>) -> Result<&Template<'a>, Error> {
        Err(Error::LoadingDisabled)
    }
}

struct NonRecursive<'a> {
    inner: &'a mut Pistachio,
    chain: Set<String>,
}

impl<'a> Loader<'static> for NonRecursive<'a> {
    fn get_partial(&mut self, name: Cow<'static, str>) -> Result<&Template<'static>, Error> {
        if !self.inner.cache || !self.inner.templates.contains_key(&name) {
            // if !self.chain.insert(name.to_string()) {
            //     return Err(Error::RecursivePartial(name.to_string()));
            // }

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
