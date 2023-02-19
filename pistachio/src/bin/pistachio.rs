use pistachio::{
    Pistachio,
    Render,
};

#[derive(Debug, Render)]
struct Body<'a> {
    content: &'a str,
}

#[derive(Debug, Render)]
struct Vars<'a> {
    title: &'a str,
    steve: &'a str,
    body: Body<'a>,
}

fn main() {
    match Pistachio::builder().directory("./examples").build() {
        Err(err) => println!("foo: {:#?}", err),
        Ok(mut pistachio) => match pistachio.get("hello-world") {
            Err(err) => println!("{}", err),
            Ok(template) => {
                println!("----------");
                println!("{:#?}", template);
                println!("----------");
                println!("{}", template.source());
                println!("----------");
                let vars = Vars {
                    title: "this is a title",
                    steve: "my man",
                    body: Body { content: "fizwop" },
                };
                let html = template.render(&vars);
                println!("{:#?}", vars);
                println!("{:?}", html);
                println!("----------");
            },
        },
    }
}
