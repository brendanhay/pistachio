use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io,
    path::PathBuf,
};

use pistachio::{
    Error,
    Loader,
    LoadingDisabled,
    Pistachio,
};
use serde::Deserialize;
use serde_json::Value;
use tempfile::TempDir;

struct View<'a, T, E>(&'a Result<T, E>);

impl<'a, T, E> fmt::Display for View<'a, T, E>
where
    T: fmt::Display,
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Ok(t) => t.fmt(f),
            Err(e) => e.fmt(f),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Spec {
    overview: String,
    tests: Vec<Test>,
}

impl Spec {
    fn run(name: &str) {
        let path = PathBuf::from(name);
        let name = path.display();
        let file = File::open(&path).expect(&format!("error reading spec {}", name));
        let spec: Self =
            serde_json::from_reader(file).expect(&format!("invalid spec json in {}", name));

        for test in spec.tests {
            test.run()
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Test {
    name: String,
    desc: String,
    data: Value,
    template: String,
    #[serde(default)]
    partials: HashMap<PathBuf, String>,
    expected: String,
}

impl Test {
    fn run(self) -> () {
        use io::Write as _;

        let tmp_dir = TempDir::new().expect("failed to create temporary directory");
        for (name, partial) in &self.partials {
            let path = tmp_dir.as_ref().join(name.with_extension("mustache"));
            File::create(&path)
                .and_then(|mut file| file.write_all(partial.as_bytes()))
                .expect("failed to write partial");
        }

        let mut pistachio = Pistachio::builder()
            .directory(&tmp_dir)
            .reloading()
            .build()
            .expect("failed to create pistachio");
        let template = pistachio
            .add(self.name.clone(), self.template)
            .expect("failed to parse template");

        let expect = self.expected;
        let actual = template.render(&self.data).map_err(|err| err.to_string());

        if actual.as_ref() != Ok(&expect) {
            let data = serde_json::to_string(&self.data).expect("unable to serialize json");

            println!("");
            println!("// Begin");
            println!("        <name> {}", &self.name);
            println!(" <description> {}", self.desc);
            println!("        <data> {}", data);
            println!("    <template> {:#?}", template);
            println!("    <expected> {}", &expect);
            println!("      <actual> {}", View(&actual));
            println!("// End");
            println!("");
        }

        assert_eq!(actual, Ok(expect));
    }
}

#[test]
fn test_spec_interpolation() {
    Spec::run("spec/specs/interpolation.json")
}

#[test]
fn test_spec_sections() {
    Spec::run("spec/specs/sections.json")
}

#[test]
fn test_spec_inverted() {
    Spec::run("spec/specs/inverted.json")
}

#[test]
fn test_spec_comments() {
    Spec::run("spec/specs/comments.json")
}

#[test]
fn test_spec_partials() {
    Spec::run("spec/specs/partials.json")
}

#[test]
fn test_spec_dynamic_names() {
    Spec::run("spec/specs/~dynamic-names.json")
}

#[test]
fn test_spec_inheritance() {
    Spec::run("spec/specs/~inheritance.json")
}

#[test]
fn test_spec_lamdas() {
    Spec::run("spec/specs/~lambdas.json")
}
