use std::{
    io,
    io::Read,
};

use pistachio::{
    vars,
    Pistachio,
};

fn main() {
    // println!("Enter mustache template:");
    // let mut source = String::new();
    // io::stdin().read_to_string(&mut source).unwrap();

    let mut pistachio = Pistachio::reload("examples", "mustache").unwrap();

    match pistachio.get("hello-world") {
        Err(err) => println!("{}", err),
        Ok(template) => {
            println!("{:#?}", template);

            let vars = vars!({
                "title": "This is a title",
                "steve": "My man"
            });

            println!("{:#?}", &vars);

            let s = template.render(&vars);

            println!("{}", s);
        },
    }
}
