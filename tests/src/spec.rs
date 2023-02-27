use std::{
    collections::{
        HashMap,
        HashSet,
    },
    fs::File,
    io,
    path::PathBuf,
};

use pistachio::Pistachio;
use serde::Deserialize;
use serde_json::Value;
use tempfile::TempDir;

#[derive(Debug, Deserialize)]
struct Spec {
    tests: Vec<Test>,
}

impl Spec {
    fn run(name: &str) {
        let path = PathBuf::from(name);
        let name = path.display();
        let file = File::open(&path).expect(&format!("error reading spec {}", name));
        let spec: Self = serde_json::from_reader(file).expect(&format!("invalid spec in {}", name));

        let ignored = (&["Deeply Nested Contexts"])
            .into_iter()
            .map(|s| s.to_string())
            .collect::<HashSet<String>>();

        for test in spec.tests {
            if ignored.contains(&test.name) {
                continue;
            }

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
            .disable_caching()
            .missing_is_false()
            .build()
            .expect("failed to create pistachio");
        let template = self.template.clone();
        let template = match pistachio.insert(&self.name, template) {
            Ok(template) => template,
            Err(err) => {
                let span = err
                    .render_span(&self.template)
                    .unwrap_or_else(|| err.to_string());

                println!("");
                println!("// Begin");
                println!("        <name> {}", &self.name);
                println!(" <description> {}", self.desc);
                println!("        <data> {}", self.data);
                println!("    <template> {}", self.template);
                println!("       <error> {}", &err);
                println!(" {}", &span);
                println!("// End");
                println!("");

                panic!("failed to parse template")
            },
        };

        println!("{:#?}", &template);

        let expect = self.expected;
        let actual = template.render(&self.data).map_err(|err| err.to_string());

        if actual.as_ref() != Ok(&expect) {
            let data = serde_json::to_string(&self.data).expect("unable to serialize json");
            let actual = match &actual {
                Ok(s) => s,
                Err(e) => e,
            };

            println!("");
            println!("// Begin");
            println!("        <name> {}", &self.name);
            println!(" <description> {}", self.desc);
            println!("        <data> {}", data);
            println!("    <template> {:#?}", template);
            println!("    <expected> {:?}", &expect);
            println!("      <actual> {:?}", &actual);
            println!("// End");
            println!("");
        }

        assert_eq!(Ok(expect), actual);
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

// #[test]
// fn test_spec_dynamic_names() {
//     Spec::run("spec/specs/~dynamic-names.json")
// }

// #[test]
// fn test_spec_inheritance() {
//     Spec::run("spec/specs/~inheritance.json")
// }

// #[test]
// fn test_spec_lamdas() {
//     Spec::run("spec/specs/~lambdas.json")
// }
