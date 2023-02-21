// use pistachio::{
//     json,
//     Pistachio,
//     Render,
// };

use serde_closure::*;
use serde_json::*;

fn main() {
    let one = 1;
    let plus_one = Fn!(|x: i32| x + one);

    println!("{:?}", to_value(plus_one));
}

// #[derive(Debug, Render)]
// struct Vars<'a> {
//     title: &'a str,
//     steve: &'a str,
// }

// fn main() {
//     let mut pistachio = Pistachio::new("examples").unwrap();

//     match pistachio.get("hello-world") {
//         Err(err) => println!("{}", err),
//         Ok(template) => {
//             println!("----------");
//             println!("{:#?}", template);
//             println!("----------");
//             println!("{}", template.source());
//             println!("----------");
//             let vars = json!({
//                 "title": "this is a title",
//                 "steve": "my man",
//                 "body": { "content": "wizzle" },
//                 "list": [
//                     {"item": {"name": "foo", "age": { "seconds": 230000 } }},
//                     {"item": {"name": "bar", "age": 70 }},
//                     {"item": {"name": "baz", "age": 39 }},
//                 ],
//             });
//             let html = template.render(&vars);
//             println!("{:#?}", vars);
//             println!("{:?}", html);
//             // println!("----------");
//             // let vars = Vars {
//             //     title: "this is a title",
//             //     steve: "my man",
//             // };
//             // let html = template.render(&vars);
//             // println!("{:#?}", vars);
//             // println!("{:?}", html);
//             // println!("----------");
//         },
//     }
// }
