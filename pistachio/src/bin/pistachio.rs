use pistachio::{
    Pistachio,
    Render,
};

#[derive(Debug, Render)]
struct Vars<'a> {
    title: &'a str,
    steve: &'a str,
}

fn main() {
    let mut pistachio = Pistachio::builder().directory("examples").build().unwrap();

    match pistachio.get("hello-world") {
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
            };
            let html = template.render(&vars);
            println!("{:#?}", vars);
            println!("{:?}", html);
            println!("----------");
        },
    }
}
