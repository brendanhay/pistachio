use std::{
    collections::{
        HashMap,
        HashSet,
    },
    fmt,
    fs::File,
    io,
    path::PathBuf,
};

use pistachio::{
    Expand,
    Pistachio,
    Render,
};
use serde::Deserialize;
use serde_json::Value;
use tempfile::TempDir;

#[derive(Debug, Deserialize)]
struct Spec {
    tests: Vec<Test<Value>>,
}

impl Spec {
    fn parse(name: &str) -> Self {
        let path = PathBuf::from(name);
        let name = path.display();
        let file = File::open(&path).expect(&format!("error reading spec {}", name));

        serde_json::from_reader(file).expect(&format!("invalid spec in {}", name))
    }

    fn run(name: &str) {
        let spec = Self::parse(name);
        let ignored = (&["Recursion"])
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
struct Test<T> {
    name: String,
    desc: String,
    data: T,
    template: String,
    #[serde(default)]
    partials: HashMap<PathBuf, String>,
    expected: String,
}

impl<T: Render + fmt::Display> Test<T> {
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
                println!("    <template> {}", self.template);
                println!("        <data> {}", self.data);
                println!("    <partials> {:?}", self.partials);
                println!("       <error> {}", &err);
                println!(" {}", &span);
                println!("// End");
                println!("");

                panic!("failed to parse template")
            },
        };

        let expect = self.expected;
        let actual = template.render(&self.data).map_err(|err| err.to_string());

        if actual.as_ref() != Ok(&expect) {
            let actual = match &actual {
                Ok(s) => s,
                Err(e) => e,
            };

            println!("");
            println!("// Begin");
            println!("        <name> {}", &self.name);
            println!(" <description> {}", self.desc);
            println!("    <template> {:?}", self.template);
            println!("        <data> {}", self.data);
            println!("    <partials> {:?}", self.partials);
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

#[test]
fn test_spec_inheritance() {
    Spec::run("spec/specs/~inheritance.json")
}

#[derive(Render)]
struct Lambda<F> {
    lambda: F,
    #[pistachio(flatten)]
    data: Value,
}

impl<F> fmt::Display for Lambda<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.data)
    }
}

#[test]
fn test_spec_lambdas() {
    for test in Spec::parse("spec/specs/~lambdas.json").tests {
        fn variable(test: Test<Value>, lambda: Box<dyn Fn() -> Expand>) {
            Test {
                name: test.name,
                desc: test.desc,
                template: test.template,
                partials: test.partials,
                expected: test.expected,
                data: Lambda {
                    lambda,
                    data: test.data,
                },
            }
            .run()
        }

        fn section(test: Test<Value>, lambda: Box<dyn Fn(&str) -> Expand>) {
            Test {
                name: test.name,
                desc: test.desc,
                template: test.template,
                partials: test.partials,
                expected: test.expected,
                data: Lambda {
                    lambda,
                    data: test.data,
                },
            }
            .run()
        }

        match &*test.name {
            "Interpolation" => {
                variable(test, Box::new(|| "world".into()));
            },

            "Interpolation - Expansion" => {
                variable(test, Box::new(|| "{{planet}}".into()));
            },

            // "Interpolation - Alternate Delimiters" => {
            //     Box::new(|_| Expand("|planet| => {{planet}}".to_string()))
            // },

            // Requires FnMut/RefCell
            // "Interpolation - Multiple Calls" => {
            //     let mut calls = 0usize;
            //     run(test, |_| {
            //         calls += 1;
            //         Expand(calls.to_string())
            //     });
            // },
            // "Escaping" => run(test, |_| ">".into()),
            //
            "Section" => {
                section(
                    test,
                    Box::new(|s| if s == "{{x}}" { "yes" } else { "no" }.into()),
                );
            },

            "Section - Expansion" => {
                section(test, Box::new(|s| (s.to_owned() + "{{planet}}" + s).into()));
            },

            // "Section - Alternate Delimiters" => {
            //     let f = |text: String| text.clone() + "{{planet}} => |planet|" + &text;
            //     ctx.insert("lambda".to_string(), Data::Fun(RefCell::new(Box::new(f))));
            // },
            //
            // "Section - Multiple Calls" => {
            //     let f = |text: String| "__".to_string() + &text + "__";
            //     ctx.insert("lambda".to_string(), Data::Fun(RefCell::new(Box::new(f))));
            // },
            //
            "Inverted Section" => {
                section(test, Box::new(|_| "".into()));
            },

            name => println!("unimplemented lambda spec: {}", name),
        }
    }
}
